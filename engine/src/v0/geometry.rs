// Geometry-related utilities

// TODO: consider using a third party library for this module
// https://docs.rs/euclid/0.20.11/euclid/
// https://docs.rs/ncollide2d/0.23.0/ncollide2d/
// https://docs.rs/bracket-geometry/0.8.1/bracket_geometry/
// Concerns are wasm compatibility / correctness / performance.

use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

#[derive(Debug)]
pub(crate) struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Copy, Clone, Deserialize, Serialize)]
pub struct BoundingBox {
    pub bottom: usize,
    pub left: usize,

    // The bounding box does not include the top right boundary
    pub top: usize,
    pub right: usize,
}

impl BoundingBox {
    pub(crate) fn rows(&self) -> std::ops::Range<usize> {
        self.bottom..self.top
    }

    pub(crate) fn cols(&self) -> std::ops::Range<usize> {
        self.left..self.right
    }

    pub(crate) fn scale(&self, factor: u8) -> Self {
        BoundingBox {
            bottom: self.bottom * (factor as usize),
            left: self.left * (factor as usize),
            top: self.top * (factor as usize),
            right: self.right * (factor as usize),
        }
    }

    pub(crate) fn size(&self) -> usize {
        let rows = self.top - self.bottom;
        let cols = self.right - self.left;

        rows * cols
    }
}

impl Position {
    pub(crate) fn distance(&self, other: &Position) -> f32 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
    }

    pub(crate) fn advance(&mut self, direction_rad: &mut f32, bounding_box: &BoundingBox) {
        self.x += direction_rad.cos();
        self.y -= direction_rad.sin();

        let top_boundary = bounding_box.top as f32;
        let left_boundary = bounding_box.left as f32;
        let bottom_boundary = bounding_box.bottom as f32;
        let right_boundary = bounding_box.right as f32;

        if self.x < left_boundary {
            self.x = 2.0 * left_boundary - self.x;
            *direction_rad = normalize_angle(PI - *direction_rad);
        }

        if self.x >= right_boundary {
            // Subtract a bit more to ensure self.x is not exactly right_boundary
            self.x = 2.0 * right_boundary - self.x - f32::EPSILON;
            *direction_rad = normalize_angle(PI - *direction_rad);
        }

        if self.y < bottom_boundary {
            self.y = 2.0 * bottom_boundary - self.y;
            *direction_rad = normalize_angle(-*direction_rad);
        }

        if self.y >= top_boundary {
            // Subtract a bit more to ensure self.y is not exactly top_boundary
            self.y = 2.0 * top_boundary - self.y - f32::EPSILON;
            *direction_rad = normalize_angle(-*direction_rad);
        }
    }
}

fn normalize_angle(t: f32) -> f32 {
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
    fn test_bounding_box() {
        let bb = BoundingBox {
            bottom: 1,
            left: 2,
            top: 6,
            right: 10,
        };

        assert_eq!(bb.rows(), 1..6);
        assert_eq!(bb.cols(), 2..10);

        assert_eq!(bb.size(), 40);
    }

    #[test]
    fn test_normalize_angle() {
        approx::assert_ulps_eq!(normalize_angle(0.0), 0.0);
        approx::assert_ulps_eq!(normalize_angle(PI), PI);
        approx::assert_ulps_eq!(normalize_angle(6.28), 6.28);
        approx::assert_ulps_eq!(normalize_angle(-PI), PI);
        approx::assert_ulps_eq!(normalize_angle(10.0), 3.716_814);
    }

    #[test]
    fn test_distance() {
        let p1 = Position { x: 1.0, y: 1.0 };
        let p2 = Position { x: 2.0, y: 2.0 };
        let p3 = Position { x: 2.0, y: 3.0 };
        approx::assert_ulps_eq!(p1.distance(&p2), std::f32::consts::SQRT_2);
        approx::assert_ulps_eq!(p2.distance(&p3), 1.0);
    }

    struct PositionAndDirection {
        pub(crate) position: Position,
        pub(crate) direction_rad: f32,
    }

    impl PositionAndDirection {
        fn advance(&mut self, world_bb: &BoundingBox) {
            self.position.advance(&mut self.direction_rad, world_bb);
        }
    }

    #[test]
    fn test_update_position_and_direction_no_collision() {
        let mut pd = PositionAndDirection {
            position: Position { x: 10.0, y: 10.0 },
            direction_rad: 0.0,
        };
        let world_size = BoundingBox {
            bottom: 0,
            left: 0,
            top: 20,
            right: 20,
        };
        let reset_position_direction = |p: &mut PositionAndDirection, t: f32| {
            p.position.x = 10.0;
            p.position.y = 10.0;
            p.direction_rad = t;
        };
        let advance = |p: &mut PositionAndDirection| {
            p.advance(&world_size);
        };

        reset_position_direction(&mut pd, 0.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 11.0);
        approx::assert_ulps_eq!(pd.position.y, 10.0);

        reset_position_direction(&mut pd, 1.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.707_107);
        approx::assert_ulps_eq!(pd.position.y, 9.292_893);

        reset_position_direction(&mut pd, 2.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.0);
        approx::assert_ulps_eq!(pd.position.y, 9.0);

        reset_position_direction(&mut pd, 3.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 9.292_893);
        approx::assert_ulps_eq!(pd.position.y, 9.292_893);

        reset_position_direction(&mut pd, 4.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 9.0);
        approx::assert_ulps_eq!(pd.position.y, 10.0);

        reset_position_direction(&mut pd, 5.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 9.292_893);
        approx::assert_ulps_eq!(pd.position.y, 10.707_107);

        reset_position_direction(&mut pd, 6.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.0);
        approx::assert_ulps_eq!(pd.position.y, 11.0);

        reset_position_direction(&mut pd, 7.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.707_107);
        approx::assert_ulps_eq!(pd.position.x, 10.707_107);
    }

    #[test]
    fn test_update_position_and_direction_left_collision() {
        let mut pd = PositionAndDirection {
            position: Position { x: 0.1, y: 10.0 },
            direction_rad: 0.0,
        };
        let world_size = BoundingBox {
            bottom: 0,
            left: 0,
            top: 20,
            right: 20,
        };
        let reset_position_direction = |p: &mut PositionAndDirection, t: f32| {
            p.position.x = 0.1;
            p.position.y = 10.0;
            p.direction_rad = t;
        };
        let advance = |p: &mut PositionAndDirection| {
            p.advance(&world_size);
        };

        reset_position_direction(&mut pd, 2.0 * PI / 3.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 0.4);
        approx::assert_ulps_eq!(pd.position.y, 9.133_975);
        approx::assert_ulps_eq!(pd.direction_rad, PI / 3.0);

        reset_position_direction(&mut pd, 5.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 0.607_106_6);
        approx::assert_ulps_eq!(pd.position.y, 10.707_107);
        approx::assert_ulps_eq!(pd.direction_rad, 7.0 * PI / 4.0);
    }

    #[test]
    fn test_update_position_and_direction_right_collision() {
        let mut pd = PositionAndDirection {
            position: Position { x: 19.9, y: 10.0 },
            direction_rad: 0.0,
        };
        let world_size = BoundingBox {
            bottom: 0,
            left: 0,
            top: 20,
            right: 20,
        };
        let reset_position_direction = |p: &mut PositionAndDirection, t: f32| {
            p.position.x = 19.9;
            p.position.y = 10.0;
            p.direction_rad = t;
        };
        let advance = |p: &mut PositionAndDirection| {
            p.advance(&world_size);
        };

        reset_position_direction(&mut pd, PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 19.392_893);
        approx::assert_ulps_eq!(pd.position.y, 9.292_893);
        approx::assert_ulps_eq!(pd.direction_rad, 3.0 * PI / 4.0);

        reset_position_direction(&mut pd, 11.0 * PI / 6.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 19.233_974);
        approx::assert_ulps_eq!(pd.position.y, 10.5);
        approx::assert_ulps_eq!(pd.direction_rad, 7.0 * PI / 6.0);
    }

    #[test]
    fn test_update_position_and_direction_top_collision() {
        let mut pd = PositionAndDirection {
            position: Position { x: 10.0, y: 0.1 },
            direction_rad: 0.0,
        };
        let world_size = BoundingBox {
            bottom: 0,
            left: 0,
            top: 20,
            right: 20,
        };
        let reset_position_direction = |p: &mut PositionAndDirection, t: f32| {
            p.position.x = 10.0;
            p.position.y = 0.1;
            p.direction_rad = t;
        };
        let advance = |p: &mut PositionAndDirection| {
            p.advance(&world_size);
        };

        reset_position_direction(&mut pd, 5.0 * PI / 6.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 9.133_974);
        approx::assert_ulps_eq!(pd.position.y, 0.399_999_8);
        approx::assert_ulps_eq!(pd.direction_rad, 7.0 * PI / 6.0);

        reset_position_direction(&mut pd, PI / 3.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.5);
        approx::assert_ulps_eq!(pd.position.y, 0.766_025_4);
        approx::assert_ulps_eq!(pd.direction_rad, 5.0 * PI / 3.0);
    }

    #[test]
    fn test_update_position_and_direction_bottom_collision() {
        let mut pd = PositionAndDirection {
            position: Position { x: 10.0, y: 19.9 },
            direction_rad: 0.0,
        };
        let world_size = BoundingBox {
            bottom: 0,
            left: 0,
            top: 20,
            right: 20,
        };
        let reset_position_direction = |p: &mut PositionAndDirection, t: f32| {
            p.position.x = 10.0;
            p.position.y = 19.9;
            p.direction_rad = t;
        };
        let advance = |p: &mut PositionAndDirection| {
            p.advance(&world_size);
        };

        reset_position_direction(&mut pd, 7.0 * PI / 6.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 9.133_975);
        approx::assert_ulps_eq!(pd.position.y, 19.6);
        approx::assert_ulps_eq!(pd.direction_rad, 5.0 * PI / 6.0);

        reset_position_direction(&mut pd, 7.0 * PI / 4.0);
        advance(&mut pd);
        approx::assert_ulps_eq!(pd.position.x, 10.707_107);
        approx::assert_ulps_eq!(pd.position.y, 19.392_895);
        approx::assert_ulps_eq!(pd.direction_rad, PI / 4.0);
    }
}
