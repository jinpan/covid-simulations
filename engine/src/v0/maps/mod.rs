pub mod simple_groceries;

use crate::v0::geometry::BoundingBox;
use anyhow::{anyhow, Result};
use pathfinding::directed::astar::astar;
use rand::{Rng, RngCore};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

pub mod loader;

pub(crate) struct Household {
    pub(crate) bounds: BoundingBox,

    pub(crate) num_people: u8,
}

pub(crate) struct Store {
    pub(crate) bounds: BoundingBox,
}

pub(crate) struct Road {
    pub(crate) bounds: BoundingBox,
}

pub struct Map {
    // Upper left is (0, 0)
    pub(crate) households: Vec<Household>,

    pub(crate) roads: Vec<Road>,

    pub(crate) stores: Vec<Store>,

    world_bb: BoundingBox,

    scale_factor: u8,
    elements: Vec<Vec<MapElement>>,

    household_to_store_path_cache: RefCell<HashMap<(usize, usize), Vec<(u16, u16)>>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum MapElement {
    Background,
    Household,
    Road,
    Store,
}

impl Map {
    fn load_lines(s: &str) -> Result<Vec<Vec<MapElement>>> {
        let parsed_ascii_map = s
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(MapElement::Background),
                        'H' => Ok(MapElement::Household),
                        'R' => Ok(MapElement::Road),
                        'S' => Ok(MapElement::Store),
                        _ => Err(anyhow!("invalid char in line: <{}>", c)),
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .rev()
            .collect::<Result<Vec<Vec<_>>>>()?;

        // Validate that the parsed map is non-empty and rectangular.
        if parsed_ascii_map.is_empty() {
            return Err(anyhow!("empty input"));
        }
        let num_cols = parsed_ascii_map[0].len();
        assert_ne!(
            num_cols, 0,
            "cannot be zero due to the above trim/strict match"
        );
        for line in parsed_ascii_map[1..].iter() {
            if line.len() != num_cols {
                return Err(anyhow!(
                    "found lines with different lengths ({} vs {})",
                    num_cols,
                    line.len()
                ));
            }
        }

        Ok(parsed_ascii_map)
    }

    fn get_bounding_boxes(
        parsed_ascii_map: &[Vec<MapElement>],
        el: MapElement,
    ) -> Vec<BoundingBox> {
        // Algorithm:
        //   Maintain set of coordinates that match the element but haven't been included in a box.
        //   Loop until set is depleted:
        //     Initialize new box from arbitrarily chosen coordinate from set
        //     Try extending the box in all four directions.
        //     If none of the directions can be extended:
        //       Then add the box to the result set
        //       Drop all coordinates within that box from the set.

        let mut remaining_coordinates = HashSet::<(usize, usize)>::new();
        for (row_idx, row) in parsed_ascii_map.iter().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                if *val == el {
                    remaining_coordinates.insert((row_idx, col_idx));
                }
            }
        }

        let map_rows = parsed_ascii_map.len();
        let map_cols = parsed_ascii_map[0].len();

        let mut boxes = vec![];

        while let Some((row, col)) = remaining_coordinates.iter().next() {
            let mut bb = BoundingBox {
                bottom: *row,
                left: *col,
                top: *row + 1,
                right: *col + 1,
            };

            loop {
                // Try extending the box to the bottom
                if bb.bottom > 0 {
                    let r = bb.bottom - 1;
                    if bb.cols().all(|c| parsed_ascii_map[r][c] == el) {
                        bb.bottom = r;
                        continue;
                    }
                }

                // Try extending the box to the left
                if bb.left > 0 {
                    let c = bb.left - 1;
                    if bb.rows().all(|r| parsed_ascii_map[r][c] == el) {
                        bb.left = c;
                        continue;
                    }
                }

                // Try extending the box to the top
                if bb.top < map_rows {
                    let r = bb.top;
                    if bb.cols().all(|c| parsed_ascii_map[r][c] == el) {
                        bb.top += 1;
                        continue;
                    }
                }

                // Try extending the box to the right
                if bb.right < map_cols {
                    let c = bb.right;
                    if bb.rows().all(|r| parsed_ascii_map[r][c] == el) {
                        bb.right += 1;
                        continue;
                    }
                }

                // Failed to extend the box in any direction
                break;
            }

            // Remove all coordinates in the box from remaining_coordinates.
            for row in bb.rows() {
                for col in bb.cols() {
                    assert_eq!(parsed_ascii_map[row][col], el);
                    remaining_coordinates.remove(&(row, col));
                }
            }

            boxes.push(bb);
        }

        boxes.sort();
        boxes
    }

    pub fn load_from_ascii_str(
        s: &str,
        scale_factor: u8,
        num_people_per_household: u8,
    ) -> Result<Self> {
        let parsed_ascii_map = Self::load_lines(s)?;

        let household_boxes = Self::get_bounding_boxes(&parsed_ascii_map, MapElement::Household);
        let households = household_boxes
            .into_iter()
            .map(|bb| Household {
                bounds: bb.scale(scale_factor),
                num_people: num_people_per_household,
            })
            .collect();

        let road_boxes = Self::get_bounding_boxes(&parsed_ascii_map, MapElement::Road);
        let roads = road_boxes
            .into_iter()
            .map(|bb| Road {
                bounds: bb.scale(scale_factor),
            })
            .collect();

        let store_boxes = Self::get_bounding_boxes(&parsed_ascii_map, MapElement::Store);
        let stores = store_boxes
            .into_iter()
            .map(|bb| Store {
                bounds: bb.scale(scale_factor),
            })
            .collect();

        let world_bb = BoundingBox {
            bottom: 0,
            left: 0,
            top: parsed_ascii_map.len() * scale_factor as usize,
            right: parsed_ascii_map[0].len() * scale_factor as usize,
        };

        Ok(Map {
            households,
            roads,
            stores,
            world_bb,
            scale_factor,
            elements: parsed_ascii_map,
            household_to_store_path_cache: RefCell::new(HashMap::new()),
        })
    }

    fn find_bb_road_intersection(&self, bb: &BoundingBox) -> Vec<((u16, u16), (u16, u16))> {
        // Returns a list of pairs of points, such that
        //   the first point of the pair is inside the bounding box
        //   the second point of the pair is on the road
        //   the first and second points are adjacent

        let mut intersections = vec![];

        // Iterate over the bottom boundary of the bounding box.
        if bb.bottom > self.world_bb.bottom {
            let row = bb.bottom - 1;
            for col in bb.cols() {
                if self.get_element(row, col) == MapElement::Road {
                    intersections.push(((bb.bottom as u16, col as u16), (row as u16, col as u16)));
                }
            }
        }
        // Iterate over the left boundary of the bounding box.
        if bb.left > self.world_bb.left {
            let col = bb.left - 1;
            for row in bb.rows() {
                if self.get_element(row, col) == MapElement::Road {
                    intersections.push(((row as u16, bb.left as u16), (row as u16, col as u16)));
                }
            }
        }

        // Iterate over the top boundary of the bounding box
        if bb.top < self.world_bb.top {
            let row = bb.top;
            for col in bb.cols() {
                if self.get_element(row, col) == MapElement::Road {
                    intersections
                        .push((((bb.top - 1) as u16, col as u16), (row as u16, col as u16)));
                }
            }
        }
        // Iterate over the right boundary of the bounding box.
        if bb.right < self.world_bb.right {
            let col = bb.right;
            for row in bb.rows() {
                if self.get_element(row, col) == MapElement::Road {
                    intersections.push((
                        (row as u16, (bb.right - 1) as u16),
                        (row as u16, col as u16),
                    ));
                }
            }
        }

        intersections
    }

    pub(crate) fn get_household_to_store_path(
        &self,
        household_idx: usize,
        store_idx: usize,
        rng: &mut dyn RngCore,
    ) -> Result<Vec<(u16, u16)>> {
        if let Some(path) = self
            .household_to_store_path_cache
            .borrow()
            .get(&(household_idx, store_idx))
        {
            return Ok(path.clone());
        }

        let household_bb = &self.households[household_idx].bounds;
        // List of (from, road) points.
        let starting_intersections = self.find_bb_road_intersection(household_bb);
        if starting_intersections.is_empty() {
            return Err(anyhow!("empty starting intersections"));
        }

        // List of (to, road) points
        let store_bb = &self.stores[store_idx].bounds;
        let ending_intersections = self.find_bb_road_intersection(store_bb);
        if ending_intersections.is_empty() {
            return Err(anyhow!("empty ending intersections"));
        }

        // Pick an arbitrary starting intersection and ending intersection.
        let starting_intersection_idx = rng.gen_range(0, starting_intersections.len());
        let starting_intersection = &starting_intersections[starting_intersection_idx];
        let starting_road_point = starting_intersection.1;

        let ending_intersection_idx = rng.gen_range(0, ending_intersections.len());
        let ending_intersection = &ending_intersections[ending_intersection_idx];
        let ending_road_point = ending_intersection.1;

        // Generate a path within the road from the starting_road_point to the ending_road_point
        let (road_path, _) = astar(
            &starting_road_point,
            |pos| {
                let mut successors = vec![];
                for (d_row, d_col) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
                    if *d_row == -1 && pos.0 == 0 {
                        continue;
                    }
                    if *d_row == 1 && pos.0 + 1 == self.world_bb.top as u16 {
                        continue;
                    }
                    if *d_col == -1 && pos.1 == 0 {
                        continue;
                    }
                    if *d_col == 1 && pos.1 + 1 == self.world_bb.right as u16 {
                        continue;
                    }

                    let candidate = ((pos.0 as i32 + d_row) as u16, (pos.1 as i32 + d_col) as u16);
                    if self.get_element(candidate.0 as usize, candidate.1 as usize)
                        == MapElement::Road
                    {
                        successors.push((candidate, 1));
                    }
                }
                successors
            },
            |pos| {
                let d_row = (pos.0 as f32) - (ending_road_point.0 as f32);
                let d_col = (pos.1 as f32) - (ending_road_point.1 as f32);

                (d_row * d_row + d_col * d_col).sqrt() as u16
            },
            |pos| *pos == ending_road_point,
        )
        .ok_or_else(|| anyhow!("failed to find road path"))?;

        let mut entire_path = vec![starting_intersection.0];
        entire_path.extend(road_path.into_iter());
        entire_path.push(ending_intersection.0);

        self.household_to_store_path_cache
            .borrow_mut()
            .insert((household_idx, store_idx), entire_path.clone());

        Ok(entire_path)
    }

    pub(crate) fn get_store_to_household_path(
        &self,
        store_idx: usize,
        household_idx: usize,
        rng: &mut dyn RngCore,
    ) -> Result<Vec<(u16, u16)>> {
        let path = self.get_household_to_store_path(household_idx, store_idx, rng)?;
        Ok(path.into_iter().rev().collect())
    }

    pub(crate) fn get_element(&self, row: usize, col: usize) -> MapElement {
        self.elements[row / self.scale_factor as usize][col / self.scale_factor as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v0::maps::simple_groceries;
    use itertools::Itertools;

    #[test]
    fn test_load_simple_groceries() -> Result<()> {
        let sg_map = Map::load_from_ascii_str(simple_groceries::MAP_ASCII_STR, 1, 1)?;

        let household_bounds = sg_map
            .households
            .iter()
            .map(|household| household.bounds)
            .sorted()
            .collect::<Vec<_>>();
        assert_eq!(household_bounds.len(), 54);
        let household_sizes = household_bounds
            .iter()
            .map(|bb| bb.size())
            .collect::<counter::Counter<_>>();
        assert_eq!(household_sizes.len(), 1);
        assert_eq!(household_sizes[&9], 54);

        assert_eq!(sg_map.roads.len(), 37);

        let store_bounds = sg_map
            .stores
            .iter()
            .map(|store| store.bounds)
            .sorted()
            .collect::<Vec<_>>();
        assert_eq!(
            store_bounds,
            vec![BoundingBox {
                bottom: 10,
                left: 10,
                top: 30,
                right: 50,
            }]
        );

        Ok(())
    }

    #[test]
    fn test_find_bb_road_intersection() -> Result<()> {
        let sg_map = Map::load_from_ascii_str(simple_groceries::MAP_ASCII_STR, 10, 1)?;
        let store = &sg_map.stores[0];

        let intersections = sg_map.find_bb_road_intersection(&store.bounds);
        assert_eq!(
            intersections,
            (290..310)
                .map(|col| ((100, col), (99, col),))
                .collect::<Vec<_>>()
        );

        let box_with_left_intersection = BoundingBox {
            bottom: 110,
            left: 60,
            top: 140,
            right: 90,
        };
        let intersections = sg_map.find_bb_road_intersection(&box_with_left_intersection);
        assert_eq!(
            intersections,
            (120..130)
                .map(|row| ((row, 60), (row, 59)))
                .collect::<Vec<_>>()
        );

        // TODO: tests for box with top/right/bottom intersection.

        Ok(())
    }
}
