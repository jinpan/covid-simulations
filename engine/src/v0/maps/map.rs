// Contains the definition of a map.

use crate::v0::geometry::BoundingBox;
use anyhow::{anyhow, Result};
use std::collections::HashSet;

struct Household {
    bounds: BoundingBox,

    num_people: u8,
}

struct Store {
    bounds: BoundingBox,
}

struct Road {
    bounds: BoundingBox,
}

struct Map {
    // Upper left is (0, 0)
    bounds: BoundingBox,

    households: Vec<Household>,

    roads: Vec<Road>,

    stores: Vec<Store>,
}

#[derive(Debug, PartialEq)]
enum MapElement {
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
        //     If none of the directions can be extended in:
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
                top_left: (*row, *col),
                bottom_right: (*row + 1, *col + 1),
            };

            loop {
                // Try extending the box to the right
                if bb.bottom_right.1 < map_cols {
                    let c = bb.bottom_right.1;
                    if bb.rows().all(|r| parsed_ascii_map[r][c] == el) {
                        bb.bottom_right.1 += 1;
                        continue;
                    }
                }

                // Try extending the box to the top
                if bb.top_left.0 > 0 {
                    let r = bb.top_left.0 - 1;
                    if bb.cols().all(|c| parsed_ascii_map[r][c] == el) {
                        bb.top_left.0 = r;
                        continue;
                    }
                }

                // Try extending the box to the left
                if bb.top_left.1 > 0 {
                    let c = bb.top_left.1 - 1;
                    if bb.rows().all(|r| parsed_ascii_map[r][c] == el) {
                        bb.top_left.1 = c;
                        continue;
                    }
                }

                // Try extending the box to the bottom
                if bb.bottom_right.0 < map_rows {
                    let r = bb.bottom_right.0;
                    if bb.cols().all(|c| parsed_ascii_map[r][c] == el) {
                        bb.bottom_right.0 += 1;
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

        boxes
    }

    fn load_from_ascii_str(scale_factor: u8, s: &str) -> Result<Self> {
        let parsed_ascii_map = Self::load_lines(s)?;

        let household_boxes = Self::get_bounding_boxes(&parsed_ascii_map, MapElement::Household);
        let households = household_boxes
            .into_iter()
            .map(|bb| Household {
                bounds: bb.scale(scale_factor),
                // TODO: stop hardcoding this as 2.
                num_people: 2,
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

        Ok(Map {
            bounds: BoundingBox {
                top_left: (0, 0),
                bottom_right: (0, 0),
            },
            households,
            roads,
            stores,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v0::maps::simple_groceries;
    use itertools::Itertools;

    #[test]
    fn test_load_simple_groceries() -> Result<()> {
        let sg_map = Map::load_from_ascii_str(1, simple_groceries::MAP_ASCII_STR)?;

        let household_bounds = sg_map
            .households
            .iter()
            .map(|household| household.bounds)
            .sorted()
            .collect::<Vec<_>>();
        assert_eq!(household_bounds.len(), 55);
        let household_sizes = household_bounds
            .iter()
            .map(|bb| bb.size())
            .collect::<counter::Counter<_>>();
        assert_eq!(household_sizes.len(), 3);
        assert_eq!(household_sizes[&6], 8);
        assert_eq!(household_sizes[&9], 44);
        assert_eq!(household_sizes[&12], 3);

        let road_bounds = sg_map
            .roads
            .iter()
            .map(|road| road.bounds)
            .sorted()
            .collect::<Vec<_>>();
        assert_eq!(
            road_bounds,
            vec![
                BoundingBox {
                    top_left: (0, 4),
                    bottom_right: (40, 5),
                },
                BoundingBox {
                    top_left: (0, 55),
                    bottom_right: (40, 56),
                },
                BoundingBox {
                    top_left: (4, 0),
                    bottom_right: (5, 60),
                },
                BoundingBox {
                    top_left: (30, 29),
                    bottom_right: (36, 31),
                },
                BoundingBox {
                    top_left: (35, 0),
                    bottom_right: (36, 60),
                },
            ]
        );

        let store_bounds = sg_map
            .stores
            .iter()
            .map(|store| store.bounds)
            .sorted()
            .collect::<Vec<_>>();
        assert_eq!(
            store_bounds,
            vec![BoundingBox {
                top_left: (10, 15),
                bottom_right: (30, 45),
            }]
        );

        Ok(())
    }
}
