use colored::*;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.txt");
    part_one(input);
    println!();
    part_two(input);
}
fn part_one(input: &str) {
    println!("Part one");
    let map = AntennaMap::new(input);
    println!();
    map.print_map();
    println!();
    let mut antinodes: HashSet<MapPosition> = HashSet::new();
    map.grid.iter().for_each(|(terrain, set)| {
        if let MapTerrain::Antenna(_) = terrain {
            let values_vec: Vec<_> = set.iter().collect(); // Collect the values into a vector for easier indexing
            for i in 0..values_vec.len() {
                for j in i + 1..values_vec.len() {
                    let first_antenna = values_vec[i];
                    let second_antenna = values_vec[j];
                    let math = AntennaMath {
                        first_antenna: *first_antenna,
                        second_antenna: *second_antenna,
                    };
                    let new_antinodes = math
                        .antinode_positions()
                        .into_iter()
                        .filter(|pos| map.in_bounds(*pos));
                    for antinode in new_antinodes {
                        antinodes.insert(antinode);
                    }
                }
            }
        };
    });
    println!();
    map.print_map_with_antinodes(&antinodes);
    println!("Antinodes: {:?}", antinodes.len());
}
fn part_two(input: &str) {
    println!("Part two");
    let map = AntennaMap::new(input);
    println!();
    map.print_map();
    println!();
    let mut antinodes: HashSet<MapPosition> = HashSet::new();
    map.grid.iter().for_each(|(terrain, set)| {
        if let MapTerrain::Antenna(_) = terrain {
            let values_vec: Vec<_> = set.iter().collect(); // Collect the values into a vector for easier indexing
            for i in 0..values_vec.len() {
                for j in i + 1..values_vec.len() {
                    let first_antenna = values_vec[i];
                    let second_antenna = values_vec[j];
                    let math = AntennaMath {
                        first_antenna: *first_antenna,
                        second_antenna: *second_antenna,
                    };
                    let new_antinodes = math
                        .resonant_antinode_positions(map.map_width(), map.map_height())
                        .into_iter()
                        .filter(|pos| map.in_bounds(*pos));
                    for antinode in new_antinodes {
                        antinodes.insert(antinode);
                    }
                }
            }
        };
    });
    println!();
    map.print_map_with_antinodes(&antinodes);
    println!("Antinodes: {:?}", antinodes.len());
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
"#;
    #[test]
    fn antenna_map() {
        let map = AntennaMap::new(INPUT);
        map.print_map();
        println!();
        println!();
        assert_eq!(map.map_width(), 12);
        assert_eq!(map.map_height(), 12);
    }
    #[test]
    fn antenna_pairs() {
        let map = AntennaMap::new(INPUT);
        map.print_map();
        let mut antinodes: HashSet<MapPosition> = HashSet::new();
        map.grid.iter().for_each(|(terrain, set)| {
            if let MapTerrain::Antenna(c) = terrain {
                let values_vec: Vec<_> = set.iter().collect(); // Collect the values into a vector for easier indexing
                for i in 0..values_vec.len() {
                    for j in i + 1..values_vec.len() {
                        let first_antenna = values_vec[i];
                        let second_antenna = values_vec[j];
                        let math = AntennaMath {
                            first_antenna: *first_antenna,
                            second_antenna: *second_antenna,
                        };
                        println!("{} antenna pair at: {}", c.to_string().blue(), math);
                        let new_antinodes = math
                            .antinode_positions()
                            .into_iter()
                            .filter(|pos| map.in_bounds(*pos));
                        for antinode in new_antinodes {
                            antinodes.insert(antinode);
                        }
                    }
                }
            };
        });
        println!();
        map.print_map_with_antinodes(&antinodes);
        assert_eq!(antinodes.len(), 14);
    }
    #[test]
    fn resonant_antinodes() {
        let map = AntennaMap::new(INPUT);
        map.print_map();
        let mut antinodes: HashSet<MapPosition> = HashSet::new();
        map.grid.iter().for_each(|(terrain, set)| {
            if let MapTerrain::Antenna(c) = terrain {
                let values_vec: Vec<_> = set.iter().collect(); // Collect the values into a vector for easier indexing
                for i in 0..values_vec.len() {
                    for j in i + 1..values_vec.len() {
                        let first_antenna = values_vec[i];
                        let second_antenna = values_vec[j];
                        let math = AntennaMath {
                            first_antenna: *first_antenna,
                            second_antenna: *second_antenna,
                        };
                        println!("{} antenna pair at: {}", c.to_string().blue(), math);
                        let new_antinodes = math
                            .resonant_antinode_positions(map.map_width(), map.map_height())
                            .into_iter()
                            .filter(|pos| map.in_bounds(*pos));
                        for antinode in new_antinodes {
                            antinodes.insert(antinode);
                        }
                    }
                }
            };
        });
        println!();
        map.print_map_with_antinodes(&antinodes);
        assert_eq!(antinodes.len(), 34);
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]
enum MapTerrain {
    Empty,
    Antenna(char),
}
impl From<char> for MapTerrain {
    fn from(c: char) -> Self {
        if c.is_alphanumeric() {
            MapTerrain::Antenna(c)
        } else {
            MapTerrain::Empty
        }
    }
}
impl Display for MapTerrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapTerrain::Empty => write!(f, "{}", ".".green()), // Add cli color to antennas
            // Add cli color to antennas
            MapTerrain::Antenna(c) => write!(f, "{}", c.to_string().blue()),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct MapPosition {
    x: i32,
    y: i32,
}
impl Display for MapPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
struct AntennaMap {
    grid: HashMap<MapTerrain, HashSet<MapPosition>>,
}
impl AntennaMap {
    fn new(input: &str) -> Self {
        let lines = input.trim().lines().collect::<Vec<_>>();
        let mut grid = HashMap::new();
        lines.iter().enumerate().for_each(|(y_pos, line)| {
            line.chars().enumerate().for_each(|(x_pos, character)| {
                let terrain = MapTerrain::from(character);
                let position = MapPosition {
                    x: x_pos as i32,
                    y: y_pos as i32,
                };
                let set = grid.entry(terrain).or_insert_with(HashSet::new);
                set.insert(position);
            });
        });
        AntennaMap { grid }
    }
    fn in_bounds(&self, position: MapPosition) -> bool {
        position.x >= 0
        && position.y >= 0
        && position.x < self.map_width() // Changed from <= to <
        && position.y < self.map_height() // Changed from <= to <
    }
    fn map_width(&self) -> i32 {
        self.grid
            .values()
            .flat_map(|set| set.iter().map(|pos| pos.x))
            .max()
            .unwrap_or(0)
            + 1 // Ensure width is at least 1 if there's any data
    }
    fn map_height(&self) -> i32 {
        self.grid
            .values()
            .flat_map(|set| set.iter().map(|pos| pos.y))
            .max()
            .unwrap_or(0)
            + 1 // Ensure height is at least 1 if there's any data
    }

    fn print_map(&self) {
        for y in 0..self.map_height() {
            for x in 0..self.map_width() {
                let position = MapPosition { x, y };
                let terrain = self
                    .grid
                    .iter()
                    .find(|(_, set)| set.contains(&position))
                    .map(|(terrain, _)| terrain)
                    .unwrap_or(&MapTerrain::Empty);
                print!("{}", terrain);
            }
            println!();
        }
    }
    fn print_map_with_antinodes(&self, antinodes: &HashSet<MapPosition>) {
        for y in 0..self.map_height() {
            for x in 0..self.map_width() {
                let position = MapPosition { x, y };
                let terrain = self
                    .grid
                    .iter()
                    .find(|(_, set)| set.contains(&position))
                    .map(|(terrain, _)| terrain)
                    .unwrap_or(&MapTerrain::Empty);
                if antinodes.contains(&position) {
                    print!("{}", "#".red());
                } else {
                    print!("{}", terrain);
                }
            }
            println!();
        }
    }
}
struct AntennaMath {
    first_antenna: MapPosition,
    second_antenna: MapPosition,
}
impl Display for AntennaMath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {})",
            self.first_antenna.to_string().green(),
            self.second_antenna.to_string().bright_green()
        )
    }
}
impl AntennaMath {
    fn distances_between(&self) -> (i32, i32) {
        let x_distance = self.first_antenna.x - self.second_antenna.x;
        let y_distance = self.first_antenna.y - self.second_antenna.y;
        (x_distance, y_distance)
    }
    fn antinode_positions(&self) -> [MapPosition; 2] {
        let (x_distance, y_distance) = self.distances_between();
        let first_antinode_x = self.first_antenna.x + x_distance;
        let first_antinode_y = self.first_antenna.y + y_distance;
        let second_antinode_x = self.second_antenna.x - x_distance;
        let second_antinode_y = self.second_antenna.y - y_distance;
        [
            MapPosition {
                x: first_antinode_x,
                y: first_antinode_y,
            },
            MapPosition {
                x: second_antinode_x,
                y: second_antinode_y,
            },
        ]
    }
    fn resonant_antinode_positions(&self, x_bound: i32, y_bound: i32) -> Vec<MapPosition> {
        let (x_distance, y_distance) = self.distances_between();
        let mut antinodes = Vec::new();
        let mut first_antinode_x = self.first_antenna.x;
        let mut first_antinode_y = self.first_antenna.y;
        let mut second_antinode_x = self.second_antenna.x;
        let mut second_antinode_y = self.second_antenna.y;
        while first_antinode_x >= 0
            && first_antinode_y >= 0
            && first_antinode_x < x_bound
            && first_antinode_y < y_bound
        {
            antinodes.push(MapPosition {
                x: first_antinode_x,
                y: first_antinode_y,
            });
            first_antinode_x += x_distance;
            first_antinode_y += y_distance;
        }
        while second_antinode_x >= 0
            && second_antinode_y >= 0
            && second_antinode_x < x_bound
            && second_antinode_y < y_bound
        {
            antinodes.push(MapPosition {
                x: second_antinode_x,
                y: second_antinode_y,
            });
            second_antinode_x -= x_distance;
            second_antinode_y -= y_distance;
        }
        antinodes
    }
}
