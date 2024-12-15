use std::{collections::HashSet, fmt::Display};

fn main() {
    println!("Hello, advent of code day twelve!");
    let garden_map = GardenMap::from(include_str!("../input.txt"));
    println!("{}", garden_map);
    println!("Fencing cost: {}", garden_map.calculate_fencing_cost());
}
#[cfg(test)]
mod tests {
    use super::*;
    const GARDEN_MAP: &str = r#"
AAAA
BBCD
BBCC
EEEC
"#;
    #[test]
    fn read_garden_map() {
        let garden_map = GardenMap::from(GARDEN_MAP);
        println!("{:?}", garden_map);
        println!("{}", garden_map);
    }
    #[test]
    fn find_crop_regions() {
        let garden_map = GardenMap::from(GARDEN_MAP);
        println!("{}", garden_map);
        assert_eq!(garden_map.map.len(), 5);
    }
    const LARGE_GARDEN_MAP: &str = r#"
    RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;
    #[test]
    fn fencing_cost() {
        let garden_map = GardenMap::from(LARGE_GARDEN_MAP);
        println!("{}", garden_map);
        println!("Regions: {}", garden_map.map.len());
        garden_map.map.iter().for_each(|region| {
            println!(
                "Region {}, Area {} , perimeter {}",
                region.crop.crop_type,
                region.region_area(),
                region.region_perimeter((garden_map.garden_width(), garden_map.garden_height()))
            );
            region.positions.iter().for_each(|position| {
                print!("({}, {})  ", position.x, position.y);
            });
            println!();
        });
        assert_eq!(garden_map.calculate_fencing_cost(), 1930);
    }
    const BULK_GARDEN_MAP: &str = r#"
    AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"#;
    #[test]
    fn bulk_fencing() {
        let garden_map = GardenMap::from(BULK_GARDEN_MAP);
        println!("{}", garden_map);
        println!("Regions: {}", garden_map.map.len());
        garden_map.map.iter().for_each(|region| {
            println!(
                "Region {}, Area {} , faces {}",
                region.crop.crop_type,
                region.region_area(),
                region.calculate_faces(&garden_map)
            );
            region.positions.iter().for_each(|position| {
                print!("({}, {})  ", position.x, position.y);
            });
            println!();
        });
        assert_eq!(garden_map.calculate_bulk_fencing_cost(), 368);
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct GardenPosition {
    x: i32,
    y: i32,
}
impl GardenPosition {
    fn is_adjacent(&self, other: &GardenPosition) -> bool {
        (self.x - other.x).abs() + (self.y - other.y).abs() == 1
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Crop {
    crop_type: char,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct CropRegion {
    crop: Crop,
    positions: Vec<GardenPosition>,
}
impl CropRegion {
    // Check if a position is part of the specified region
    fn is_part_of_region(&self, position: &GardenPosition) -> bool {
        self.positions.contains(position)
    }
    fn region_area(&self) -> i32 {
        self.positions.len() as i32
    }
    fn region_perimeter(&self, bounds: (i32, i32)) -> i32 {
        let mut perimeter = 0;

        // Directions: Up, Down, Left, Right
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for position in &self.positions {
            for &(dx, dy) in &directions {
                let neighbor = GardenPosition {
                    x: position.x + dx,
                    y: position.y + dy,
                };
                let is_inbounds = neighbor.x >= 0
                    && neighbor.x < bounds.0
                    && neighbor.y >= 0
                    && neighbor.y < bounds.1;
                // Check if neighbor is outside the garden or not part of the same region
                if !self.is_part_of_region(&neighbor) || !is_inbounds {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }
    fn run_in_direction(
        &self,
        visited: &mut HashSet<GardenPosition>,
        position: &GardenPosition,
        direction: (i32, i32),
    ) -> bool {
        let mut new_pos = *position;
        while self.is_part_of_region(&new_pos) {
            if visited.contains(&new_pos) {
                return false;
            }
            visited.insert(new_pos);
            new_pos.x += direction.0;
            new_pos.y += direction.1;
        }
        return true;
    }
    fn calculate_faces(&self, region: &GardenMap) -> i32 {
        let mut total_faces = 0;

        // To track visited positions, we avoid double-counting runs
        let mut visited: HashSet<GardenPosition> = HashSet::new();

        // Check all positions in the region
        for position in &self.positions {
            if !visited.contains(position) {
                // If the position hasn't been visited yet, check its "run" in the two directions
                let mut new_faces = 0;

                // Check horizontally (right direction)
                if region.is_within_bounds(&position) && self.is_part_of_region(&position) {
                    if self.run_in_direction(&mut visited, &position, (0, 1)) {
                        new_faces += 1;
                    }
                    if self.run_in_direction(&mut visited, &position, (0, -1)) {
                        new_faces += 1;
                    }
                }
                // add horizontal or vertical face
                total_faces += new_faces;
            }
        }

        total_faces
    }
}
#[derive(Debug)]
struct GardenMap {
    map: HashSet<CropRegion>,
}
impl GardenMap {
    fn garden_width(&self) -> i32 {
        self.map
            .iter()
            .map(|region| {
                region
                    .positions
                    .iter()
                    .map(|position| position.x)
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
            + 1
    }
    fn garden_height(&self) -> i32 {
        self.map
            .iter()
            .map(|region| {
                region
                    .positions
                    .iter()
                    .map(|position| position.y)
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
            + 1
    }
    fn calculate_fencing_cost(&self) -> i32 {
        let garden_width = self.garden_width();
        let garden_height = self.garden_height();
        self.map.iter().fold(0, |mut total_cost, region| {
            let perimeter = region.region_perimeter((garden_width, garden_height));
            let area = region.region_area();
            total_cost += perimeter * area;
            total_cost
        })
    }
    fn calculate_bulk_fencing_cost(&self) -> i32 {
        self.map.iter().fold(0, |mut total_cost, region| {
            let area = region.region_area();
            let faces = region.calculate_faces(self);
            total_cost += area * faces;
            total_cost
        })
    }
    fn is_within_bounds(&self, position: &GardenPosition) -> bool {
        let width = self.garden_width();
        let height = self.garden_height();
        position.x >= 0 && position.x < width && position.y >= 0 && position.y < height
    }
}
impl From<&str> for GardenMap {
    fn from(garden_map: &str) -> Self {
        let trimmed_garden_map = garden_map.trim();
        let mut garden_map = GardenMap {
            map: HashSet::new(),
        };

        trimmed_garden_map
            .lines()
            .enumerate()
            .for_each(|(y, line)| {
                line.chars().enumerate().for_each(|(x, crop_type)| {
                    let garden_position = GardenPosition {
                        x: x as i32,
                        y: y as i32,
                    };
                    let crop = Crop { crop_type };

                    // Find if there are any existing adjacent regions
                    let mut adjacent_regions: Vec<CropRegion> = garden_map
                        .map
                        .iter()
                        .filter(|region| {
                            region.crop == crop
                                && region
                                    .positions
                                    .iter()
                                    .any(|position| position.is_adjacent(&garden_position))
                        })
                        .cloned()
                        .collect();

                    if adjacent_regions.is_empty() {
                        // No adjacent region, create a new one
                        garden_map.map.insert(CropRegion {
                            crop,
                            positions: vec![garden_position],
                        });
                    } else {
                        // Merge all adjacent regions into a single one
                        let mut new_positions = vec![garden_position];
                        let mut regions_to_remove = Vec::new();

                        for region in adjacent_regions {
                            new_positions.extend(region.positions.clone());
                            regions_to_remove.push(region);
                        }

                        // Remove the old regions and insert the new merged one
                        for region in regions_to_remove {
                            garden_map.map.remove(&region);
                        }

                        garden_map.map.insert(CropRegion {
                            crop,
                            positions: new_positions,
                        });
                    }
                });
            });

        garden_map
    }
}

impl Display for GardenMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let garden_width = self.garden_width();
        let garden_height = self.garden_height();
        for y in 0..garden_height {
            for x in 0..garden_width {
                let crop_region = self.map.iter().find(|region| {
                    region
                        .positions
                        .iter()
                        .any(|position| position.x == x && position.y == y)
                });
                if let Some(crop_region) = crop_region {
                    write!(f, "{}", crop_region.crop.crop_type)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
