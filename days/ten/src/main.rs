use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use colored::Colorize;

fn main() {
    println!("Hello, world!");
    let input_str = include_str!("../input.txt");
    let mut trail_rows = input_str
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    trail_rows.retain(|row| !row.is_empty());
    let mut trail_map = TrailMap::new();
    trail_rows.iter().enumerate().for_each(|(row, trail_cols)| {
        trail_cols.iter().enumerate().for_each(|(col, trail)| {
            trail_map.grid.insert(
                TrailPosition {
                    row: row as i32,
                    col: col as i32,
                },
                TrailTerrain::from(*trail),
            );
        });
    });
    println!("{}", trail_map);
    let pleasant_trail_count = trail_map.check_trailhead_pleasantness();
    println!("Pleasant trail count: {}", pleasant_trail_count);

    let pleasant_trail_rating = trail_map.check_trail_ratings();
    println!("Pleasant trail rating: {}", pleasant_trail_rating);
}
#[cfg(test)]
mod tests {

    use crate::{TrailMap, TrailPosition, TrailTerrain};

    const TEST_TRAILS: &str = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;
    #[test]
    fn read_trail_map() {
        let mut trail_rows = TEST_TRAILS
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        trail_rows.retain(|row| !row.is_empty());
        let mut trail_map = TrailMap::new();
        trail_rows.iter().enumerate().for_each(|(row, trail_cols)| {
            trail_cols.iter().enumerate().for_each(|(col, trail)| {
                trail_map.grid.insert(
                    TrailPosition {
                        row: row as i32,
                        col: col as i32,
                    },
                    TrailTerrain::from(*trail),
                );
            });
        });
        println!("{}", trail_map);
        let pleasant_trail_count = trail_map.check_trailhead_pleasantness();
        println!("Pleasant trail count: {}", pleasant_trail_count);
        assert_eq!(pleasant_trail_count, 36);
    }
    #[test]
    fn check_trail_ratings() {
        let mut trail_rows = TEST_TRAILS
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        trail_rows.retain(|row| !row.is_empty());
        let mut trail_map = TrailMap::new();
        trail_rows.iter().enumerate().for_each(|(row, trail_cols)| {
            trail_cols.iter().enumerate().for_each(|(col, trail)| {
                trail_map.grid.insert(
                    TrailPosition {
                        row: row as i32,
                        col: col as i32,
                    },
                    TrailTerrain::from(*trail),
                );
            });
        });
        println!("{}", trail_map);
        let pleasant_trail_count = trail_map.check_trail_ratings();
        println!("Pleasant trail count: {}", pleasant_trail_count);
        assert_eq!(pleasant_trail_count, 81);
    }
}
enum TrailDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct TrailPosition {
    col: i32,
    row: i32,
}
impl TrailPosition {
    fn move_position(&self, direction: &TrailDirection) -> TrailPosition {
        match direction {
            TrailDirection::Up => TrailPosition {
                row: self.row - 1,
                col: self.col,
            },
            TrailDirection::Down => TrailPosition {
                row: self.row + 1,
                col: self.col,
            },
            TrailDirection::Left => TrailPosition {
                row: self.row,
                col: self.col - 1,
            },
            TrailDirection::Right => TrailPosition {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum TrailTerrain {
    TrailHead,
    Slope(u32),
    Peak,
}
impl TrailTerrain {
    fn has_pleasant_slope(&self, next: &TrailTerrain) -> bool {
        match self {
            TrailTerrain::TrailHead => {
                // From TrailHead, we can only go to a Slope with height 1
                if *next == TrailTerrain::Slope(1) {
                    return true;
                }
                false
            }
            TrailTerrain::Slope(height) => {
                if let TrailTerrain::Slope(next_height) = next {
                    // Valid transition is from one slope to the next (height + 1)
                    if *next_height == height + 1 {
                        return true;
                    }
                }
                if *next == TrailTerrain::Peak && *height == 8 {
                    // If the current slope is 8, we can transition to a Peak
                    return true;
                }
                false
            }
            TrailTerrain::Peak => {
                // Reached a Peak, valid end point for any path
                false
            }
        }
    }
}
impl From<char> for TrailTerrain {
    fn from(c: char) -> Self {
        match c {
            '0' => TrailTerrain::TrailHead,
            '1'..='8' => TrailTerrain::Slope(c.to_digit(10).unwrap()),
            '9' => TrailTerrain::Peak,
            _ => panic!("Invalid terrain character"),
        }
    }
}
impl Display for TrailTerrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrailTerrain::TrailHead => write!(f, "{}", 0.to_string().red()),
            TrailTerrain::Slope(height) => write!(f, "{}", height.to_string().yellow()),
            TrailTerrain::Peak => write!(f, "{}", 9.to_string().green()),
        }
    }
}

#[derive(Debug)]
struct TrailMap {
    grid: HashMap<TrailPosition, TrailTerrain>,
}
impl TrailMap {
    fn trailheads(&self) -> Vec<TrailPosition> {
        self.grid
            .iter()
            .filter(|(_, terrain)| **terrain == TrailTerrain::TrailHead)
            .map(|(pos, _)| *pos)
            .collect()
    }
    fn peaks(&self) -> HashSet<TrailPosition> {
        self.grid
            .iter()
            .filter(|(_, terrain)| **terrain == TrailTerrain::Peak)
            .map(|(pos, _)| *pos)
            .collect()
    }
    fn bfs(&self, start: TrailPosition) -> HashSet<Vec<TrailPosition>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut unique_paths: HashSet<Vec<TrailPosition>> = HashSet::new();

        queue.push_back((start, vec![start])); // Start with the first position
        visited.insert(start);

        let directions = vec![
            TrailDirection::Up,
            TrailDirection::Down,
            TrailDirection::Left,
            TrailDirection::Right,
        ];

        while let Some((current_pos, path)) = queue.pop_front() {
            if let Some(current_terrain) = self.grid.get(&current_pos) {
                // Check if the current position is a peak (valid path end)
                // Explore the neighboring positions
                if current_terrain == &TrailTerrain::Peak {
                    unique_paths.insert(path.clone());
                }
                for dir in directions.iter() {
                    let new_pos = current_pos.move_position(dir);
                    // Skip if there's no terrain or the position is already visited
                    if let Some(new_terrain) = self.grid.get(&new_pos) {
                        // Check if moving from the current terrain to the new terrain is valid
                        if current_terrain.has_pleasant_slope(new_terrain) {
                            visited.insert(new_pos);
                            let mut new_path = path.clone();
                            new_path.push(new_pos); // Add the new position to the path
                            queue.push_back((new_pos, new_path));
                        }
                    }
                }
            }
        }
        unique_paths
    }
    fn check_trailhead_pleasantness(&self) -> u32 {
        let mut found_path = 0;

        let start_positions = self.trailheads();
        for &start in &start_positions {
            let unique_peaks: HashSet<TrailPosition> = self
                .bfs(start)
                .iter()
                .filter_map(|path| path.last())
                .copied()
                .collect();
            found_path += unique_peaks.len();
        }
        found_path as u32
    }
    fn check_trail_ratings(&self) -> u32 {
        let mut found_path = 0;

        let start_positions = self.trailheads();
        for &start in &start_positions {
            found_path += self.bfs(start).len();
        }
        found_path as u32
    }

    fn grid_width(&self) -> i32 {
        self.grid.keys().map(|pos| pos.col).max().unwrap_or(0) + 1
    }
    fn grid_height(&self) -> i32 {
        self.grid.keys().map(|pos| pos.row).max().unwrap_or(0) + 1
    }
    fn new() -> Self {
        TrailMap {
            grid: HashMap::new(),
        }
    }
}
impl Display for TrailMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.grid_height() {
            for col in 0..self.grid_width() {
                let pos = TrailPosition { row, col };
                if let Some(height) = self.grid.get(&pos) {
                    write!(f, "{}", height)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
const PLEASANT_TRAIL: [TrailTerrain; 11] = [
    TrailTerrain::TrailHead,
    TrailTerrain::Slope(1),
    TrailTerrain::Slope(2),
    TrailTerrain::Slope(3),
    TrailTerrain::Slope(4),
    TrailTerrain::Slope(5),
    TrailTerrain::Slope(6),
    TrailTerrain::Slope(7),
    TrailTerrain::Slope(8),
    TrailTerrain::Slope(9),
    TrailTerrain::Peak,
];
