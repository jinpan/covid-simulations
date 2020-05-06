use itertools::Itertools;
use rand::Rng;
use std::f32::consts::PI;
use std::time::Duration;

const INFECTION_RADIUS: u16 = 6;
const INFECTIOUS_PERIOD_TICKS: usize = 20;

enum DiseaseState {
    Susceptible,
    Infectious(usize), // Tick of when the person was infected
    Recovered,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

fn distance(p1: &Position, p2: &Position) -> f32 {
    ((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)).sqrt()
}

struct Person {
    disease_state: DiseaseState,
    position: Position,
    direction_rad: f32,
}

impl Person {
    fn try_infect(&mut self, tick: usize) {
        match self.disease_state {
            DiseaseState::Susceptible => self.disease_state = DiseaseState::Infectious(tick),
            DiseaseState::Infectious(_) => {}
            DiseaseState::Recovered => {
                // For now, assume a simple SIR model where recovered cannot transition back to
                // Susceptible.
            }
        }
    }

    // Advance the position of the person by 1, and bounce off of the boundaries
    // of the world.
    fn update_position(&mut self, world_size: &(u16, u16)) {
        self.position.x += self.direction_rad.cos();
        self.position.y -= self.direction_rad.sin();

        if self.position.x < 0.0 {
            self.position.x = -self.position.x;
            self.direction_rad = normalize_angle(PI - self.direction_rad);
        }

        let right_boundary = world_size.0 as f32;
        if self.position.x > right_boundary {
            self.position.x = 2.0 * right_boundary - self.position.x;
            self.direction_rad = normalize_angle(PI - self.direction_rad);
        }

        if self.position.y < 0.0 {
            self.position.y = -self.position.y;
            self.direction_rad = normalize_angle(-self.direction_rad);
        }

        let bottom_boundary = world_size.0 as f32;
        if self.position.y > bottom_boundary {
            self.position.y = 2.0 * bottom_boundary - self.position.y;
            self.direction_rad = normalize_angle(-self.direction_rad);
        }
    }

    // TODO: is the physics on this right?  This seems intuitively plausible, but have not derived
    // this.  The exact kinematics don't really matter here, but it would be nice to get it right.
    fn update_angles_on_collision(&mut self, other: &mut Person) {
        let tmp = self.direction_rad;
        self.direction_rad = other.direction_rad;
        other.direction_rad = tmp;
    }
}

struct World<'a, R>
where
    R: rand::Rng + ?Sized,
{
    size: (u16, u16),
    people: Vec<Person>,

    rng: &'a mut R,
}

fn normalize_angle(t: f32) -> f32 {
    let rem = t % (2.0 * PI);
    if rem < 0. {
        2.0 * PI + rem
    } else {
        rem
    }
}

impl<'a, R> World<'a, R>
where
    R: rand::Rng + ?Sized,
{
    fn new(
        rng: &'a mut R,
        size: (u16, u16),
        num_people: usize,
        num_initially_infected: usize,
    ) -> Self {
        assert!(num_people >= num_initially_infected);

        let people = (0..num_people)
            .map(|i| {
                let disease_state = if i < num_initially_infected {
                    DiseaseState::Infectious(0)
                } else {
                    DiseaseState::Susceptible
                };

                let x = rng.gen_range(0.0, size.0 as f32);
                let y = rng.gen_range(0.0, size.1 as f32);

                let direction_rad = rng.gen_range(0.0, 2.0 * PI);

                Person {
                    disease_state,
                    position: Position { x, y },
                    direction_rad,
                }
            })
            .collect();

        World { size, people, rng }
    }

    fn step(&mut self, tick: usize) {
        // Step 1: advance all the people
        let size = &self.size;
        self.people.iter_mut().for_each(|p| {
            p.update_position(size);
        });

        // Step 2: advance infectious states to recovered
        self.people.iter_mut().for_each(|p| {
            if let DiseaseState::Infectious(start_tick) = p.disease_state {
                if tick - start_tick > INFECTIOUS_PERIOD_TICKS {
                    p.disease_state = DiseaseState::Recovered;
                }
            }
        });

        // Step 3: identify collisions
        // TODO: instead of a N^2 loop, use some index structure (BTreeMap?)
        for i in 0..(self.people.len() - 1) {
            let (left, right) = self.people.split_at_mut(i + 1);
            let p0 = left.last_mut().unwrap();

            for p1 in right.iter_mut() {
                if distance(&p0.position, &p1.position) >= INFECTION_RADIUS as f32 {
                    continue;
                }

                // Step 3.1: Update trajectories for collisions
                p0.update_angles_on_collision(p1);

                // Step 3.2: Update disease state for collisions
                if let DiseaseState::Infectious(_) = p0.disease_state {
                    p1.try_infect(tick);
                }
                if let DiseaseState::Infectious(_) = p1.disease_state {
                    p0.try_infect(tick);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_angle() {
        approx::assert_ulps_eq!(normalize_angle(0.0), 0.0);
        approx::assert_ulps_eq!(normalize_angle(3.14), 3.14);
        approx::assert_ulps_eq!(normalize_angle(6.28), 6.28);
        approx::assert_ulps_eq!(normalize_angle(-3.14), 3.1431854);
        approx::assert_ulps_eq!(normalize_angle(10.0), 3.7168145);
    }

    #[test]
    fn test_distance() {
        let p1 = Position { x: 1.0, y: 1.0 };
        let p2 = Position { x: 2.0, y: 2.0 };
        let p3 = Position { x: 2.0, y: 3.0 };
        approx::assert_ulps_eq!(distance(&p1, &p2), 1.4142135);
        approx::assert_ulps_eq!(distance(&p2, &p3), 1.0);
    }

    #[test]
    fn test_update_person_position_no_collision() {
        let mut p0 = Person {
            disease_state: DiseaseState::Susceptible,
            position: Position { x: 10.0, y: 10.0 },
            direction_rad: 0.0,
        };
        let reset_position_direction = |p: &mut Person, t: f32| {
            p.position.x = 10.0;
            p.position.y = 10.0;
            p.direction_rad = t;
        };
        let update_position = |p: &mut Person| {
            p.update_position(&(20, 20));
        };

        reset_position_direction(&mut p0, 0.0 * PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 11.0);
        approx::assert_ulps_eq!(p0.position.y, 10.0);

        reset_position_direction(&mut p0, 1.0 * PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 10.707107);
        approx::assert_ulps_eq!(p0.position.y, 9.292893);

        reset_position_direction(&mut p0, 2.0 * PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 10.0);
        approx::assert_ulps_eq!(p0.position.y, 9.0);

        reset_position_direction(&mut p0, 3.0 * PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 9.292893);
        approx::assert_ulps_eq!(p0.position.y, 9.292893);

        reset_position_direction(&mut p0, 4.0 * PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 9.0);
        approx::assert_ulps_eq!(p0.position.y, 10.0);

        reset_position_direction(&mut p0, 5.0 * PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 9.292893);
        approx::assert_ulps_eq!(p0.position.y, 10.707107);

        reset_position_direction(&mut p0, 6.0 * PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 10.0);
        approx::assert_ulps_eq!(p0.position.y, 11.0);

        reset_position_direction(&mut p0, 7.0 * PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 10.707107);
        approx::assert_ulps_eq!(p0.position.x, 10.707107);
    }

    #[test]
    fn test_update_person_position_left_collision() {
        let mut p0 = Person {
            disease_state: DiseaseState::Susceptible,
            position: Position { x: 0.1, y: 10.0 },
            direction_rad: 0.0,
        };
        let reset_position_direction = |p: &mut Person, t: f32| {
            p.position.x = 0.1;
            p.position.y = 10.0;
            p.direction_rad = t;
        };
        let update_position = |p: &mut Person| {
            p.update_position(&(20, 20));
        };

        reset_position_direction(&mut p0, 2.0 * PI / 3.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 0.4);
        approx::assert_ulps_eq!(p0.position.y, 9.133975);
        approx::assert_ulps_eq!(p0.direction_rad, PI / 3.0);

        reset_position_direction(&mut p0, 5.0 * PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 0.6071066);
        approx::assert_ulps_eq!(p0.position.y, 10.707107);
        approx::assert_ulps_eq!(p0.direction_rad, 7.0 * PI / 4.0);
    }

    #[test]
    fn test_update_person_position_right_collision() {
        let mut p0 = Person {
            disease_state: DiseaseState::Susceptible,
            position: Position { x: 19.9, y: 10.0 },
            direction_rad: 0.0,
        };
        let reset_position_direction = |p: &mut Person, t: f32| {
            p.position.x = 19.9;
            p.position.y = 10.0;
            p.direction_rad = t;
        };
        let update_position = |p: &mut Person| {
            p.update_position(&(20, 20));
        };

        reset_position_direction(&mut p0, PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 19.392893);
        approx::assert_ulps_eq!(p0.position.y, 9.292893);
        approx::assert_ulps_eq!(p0.direction_rad, 3.0 * PI / 4.0);

        reset_position_direction(&mut p0, 11.0 * PI / 6.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 19.233974);
        approx::assert_ulps_eq!(p0.position.y, 10.5);
        approx::assert_ulps_eq!(p0.direction_rad, 7.0 * PI / 6.0);
    }

    #[test]
    fn test_update_person_position_top_collision() {
        let mut p0 = Person {
            disease_state: DiseaseState::Susceptible,
            position: Position { x: 10.0, y: 0.1 },
            direction_rad: 0.0,
        };
        let reset_position_direction = |p: &mut Person, t: f32| {
            p.position.x = 10.0;
            p.position.y = 0.1;
            p.direction_rad = t;
        };
        let update_position = |p: &mut Person| {
            p.update_position(&(20, 20));
        };

        reset_position_direction(&mut p0, 5.0 * PI / 6.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 9.133974);
        approx::assert_ulps_eq!(p0.position.y, 0.39999983);
        approx::assert_ulps_eq!(p0.direction_rad, 7.0 * PI / 6.0);

        reset_position_direction(&mut p0, PI / 3.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 10.5);
        approx::assert_ulps_eq!(p0.position.y, 0.7660254);
        approx::assert_ulps_eq!(p0.direction_rad, 5.0 * PI / 3.0);
    }

    #[test]
    fn test_update_person_position_bottom_collision() {
        let mut p0 = Person {
            disease_state: DiseaseState::Susceptible,
            position: Position { x: 10.0, y: 19.9 },
            direction_rad: 0.0,
        };
        let reset_position_direction = |p: &mut Person, t: f32| {
            p.position.x = 10.0;
            p.position.y = 19.9;
            p.direction_rad = t;
        };
        let update_position = |p: &mut Person| {
            p.update_position(&(20, 20));
        };

        reset_position_direction(&mut p0, 7.0 * PI / 6.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 9.133975);
        approx::assert_ulps_eq!(p0.position.y, 19.6);
        approx::assert_ulps_eq!(p0.direction_rad, 5.0 * PI / 6.0);

        reset_position_direction(&mut p0, 7.0 * PI / 4.0);
        update_position(&mut p0);
        approx::assert_ulps_eq!(p0.position.x, 10.707107);
        approx::assert_ulps_eq!(p0.position.y, 19.392895);
        approx::assert_ulps_eq!(p0.direction_rad, PI / 4.0);
    }

    #[test]
    fn test_update_person_angles_on_collision() {
        let mut p0 = Person {
            disease_state: DiseaseState::Susceptible,
            position: Position { x: 10.0, y: 19.9 },
            direction_rad: 0.0,
        };
        let mut p1 = Person {
            disease_state: DiseaseState::Susceptible,
            position: Position { x: 10.0, y: 19.9 },
            direction_rad: PI / 2.0,
        };

        p0.update_angles_on_collision(&mut p1);
        approx::assert_ulps_eq!(p0.direction_rad, PI / 2.0);
        approx::assert_ulps_eq!(p1.direction_rad, 0.0);
    }

    #[test]
    fn initialize_world() {
        // TODO: fix the seed and check for the counts of people by DiseaseState.
        let mut rng = rand::thread_rng();

        let mut world = World::new(&mut rng, (400, 300), 200, 1);
        for tick in 1..1000 {
            world.step(tick);
        }
    }
}
