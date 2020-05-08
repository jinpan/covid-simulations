use std::f32::consts::PI;

pub(crate) struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub(crate) struct PositionAndDirection {
    pub(crate) position: Position,
    pub(crate) direction_rad: f32,
}

impl Position {
    pub(crate) fn distance(&self, other: &Position) -> f32 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
    }
}

impl PositionAndDirection {
    pub(crate) fn advance(&mut self, world_size: &(u16, u16)) {
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

        let bottom_boundary = world_size.1 as f32;
        if self.position.y > bottom_boundary {
            self.position.y = 2.0 * bottom_boundary - self.position.y;
            self.direction_rad = normalize_angle(-self.direction_rad);
        }
    }
}

pub(crate) fn normalize_angle(t: f32) -> f32 {
    let rem = t % (2.0 * PI);
    if rem < 0. {
        2.0 * PI + rem
    } else {
        rem
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
        approx::assert_ulps_eq!(p1.distance(&p2), 1.4142135);
        approx::assert_ulps_eq!(p2.distance(&p3), 1.0);
    }

    #[test]
    fn test_update_position_and_direction_no_collision() {
        let mut pd = PositionAndDirection {
            position: Position { x: 10.0, y: 10.0 },
            direction_rad: 0.0,
        };
        let reset_position_direction = |p: &mut PositionAndDirection, t: f32| {
            p.position.x = 10.0;
            p.position.y = 10.0;
            p.direction_rad = t;
        };
        let advance = |p: &mut PositionAndDirection| {
            p.advance(&(20, 20));
        };

        reset_position_direction(&mut pd, 0.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 11.0);
        approx::assert_ulps_eq!(pd.position.y, 10.0);

        reset_position_direction(&mut pd, 1.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.707107);
        approx::assert_ulps_eq!(pd.position.y, 9.292893);

        reset_position_direction(&mut pd, 2.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.0);
        approx::assert_ulps_eq!(pd.position.y, 9.0);

        reset_position_direction(&mut pd, 3.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 9.292893);
        approx::assert_ulps_eq!(pd.position.y, 9.292893);

        reset_position_direction(&mut pd, 4.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 9.0);
        approx::assert_ulps_eq!(pd.position.y, 10.0);

        reset_position_direction(&mut pd, 5.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 9.292893);
        approx::assert_ulps_eq!(pd.position.y, 10.707107);

        reset_position_direction(&mut pd, 6.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.0);
        approx::assert_ulps_eq!(pd.position.y, 11.0);

        reset_position_direction(&mut pd, 7.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.707107);
        approx::assert_ulps_eq!(pd.position.x, 10.707107);
    }

    #[test]
    fn test_update_position_and_direction_left_collision() {
        let mut pd = PositionAndDirection {
            position: Position { x: 0.1, y: 10.0 },
            direction_rad: 0.0,
        };
        let reset_position_direction = |p: &mut PositionAndDirection, t: f32| {
            p.position.x = 0.1;
            p.position.y = 10.0;
            p.direction_rad = t;
        };
        let advance = |p: &mut PositionAndDirection| {
            p.advance(&(20, 20));
        };

        reset_position_direction(&mut pd, 2.0 * PI / 3.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 0.4);
        approx::assert_ulps_eq!(pd.position.y, 9.133975);
        approx::assert_ulps_eq!(pd.direction_rad, PI / 3.0);

        reset_position_direction(&mut pd, 5.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 0.6071066);
        approx::assert_ulps_eq!(pd.position.y, 10.707107);
        approx::assert_ulps_eq!(pd.direction_rad, 7.0 * PI / 4.0);
    }

    #[test]
    fn test_update_position_and_direction_right_collision() {
        let mut pd = PositionAndDirection {
            position: Position { x: 19.9, y: 10.0 },
            direction_rad: 0.0,
        };
        let reset_position_direction = |p: &mut PositionAndDirection, t: f32| {
            p.position.x = 19.9;
            p.position.y = 10.0;
            p.direction_rad = t;
        };
        let advance = |p: &mut PositionAndDirection| {
            p.advance(&(20, 20));
        };

        reset_position_direction(&mut pd, PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 19.392893);
        approx::assert_ulps_eq!(pd.position.y, 9.292893);
        approx::assert_ulps_eq!(pd.direction_rad, 3.0 * PI / 4.0);

        reset_position_direction(&mut pd, 11.0 * PI / 6.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 19.233974);
        approx::assert_ulps_eq!(pd.position.y, 10.5);
        approx::assert_ulps_eq!(pd.direction_rad, 7.0 * PI / 6.0);
    }

    #[test]
    fn test_update_position_and_direction_top_collision() {
        let mut pd = PositionAndDirection {
            position: Position { x: 10.0, y: 0.1 },
            direction_rad: 0.0,
        };
        let reset_position_direction = |p: &mut PositionAndDirection, t: f32| {
            p.position.x = 10.0;
            p.position.y = 0.1;
            p.direction_rad = t;
        };
        let advance = |p: &mut PositionAndDirection| {
            p.advance(&(20, 20));
        };

        reset_position_direction(&mut pd, 5.0 * PI / 6.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 9.133974);
        approx::assert_ulps_eq!(pd.position.y, 0.39999983);
        approx::assert_ulps_eq!(pd.direction_rad, 7.0 * PI / 6.0);

        reset_position_direction(&mut pd, PI / 3.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.5);
        approx::assert_ulps_eq!(pd.position.y, 0.7660254);
        approx::assert_ulps_eq!(pd.direction_rad, 5.0 * PI / 3.0);
    }

    #[test]
    fn test_update_position_and_direction_bottom_collision() {
        let mut pd = PositionAndDirection {
            position: Position { x: 10.0, y: 19.9 },
            direction_rad: 0.0,
        };
        let reset_position_direction = |p: &mut PositionAndDirection, t: f32| {
            p.position.x = 10.0;
            p.position.y = 19.9;
            p.direction_rad = t;
        };
        let advance = |p: &mut PositionAndDirection| {
            p.advance(&(20, 20));
        };

        reset_position_direction(&mut pd, 7.0 * PI / 6.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 9.133975);
        approx::assert_ulps_eq!(pd.position.y, 19.6);
        approx::assert_ulps_eq!(pd.direction_rad, 5.0 * PI / 6.0);

        reset_position_direction(&mut pd, 7.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.707107);
        approx::assert_ulps_eq!(pd.position.y, 19.392895);
        approx::assert_ulps_eq!(pd.direction_rad, PI / 4.0);
    }
}
