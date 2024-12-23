use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Display, Formatter},
};

use colored::Colorize;
const TEST_STRING: &str = r#"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;

fn main() {
    println!("Hello, world!");
    let mut track = Track::try_from(TEST_STRING.trim()).unwrap();
    println!("{}", track);
    let paths = track.find_all_paths();
    for path in paths {
        let mut track_clone = Track::try_from(TEST_STRING.trim()).unwrap();
        for position in path {
            track_clone.tiles.insert(position, TrackTile::Raced);
        }
        println!("{}", track_clone);
    }
}

struct Track {
    tiles: HashMap<TrackPosition, TrackTile>,
    start: TrackPosition,
    end: TrackPosition,
}
impl Track {
    fn cheat_neighbours(&self, position: TrackPosition) -> Vec<TrackPosition> {
        let mut neighbours = Vec::new();
        let x = position.x as i32;
        let y = position.y as i32;
        let directions = vec![(0, -2), (0, 2), (-2, 0), (2, 0)];
        for (dx, dy) in directions {
            let new_position = TrackPosition {
                x: (x + dx) as usize,
                y: (y + dy) as usize,
            };
            if let Some(tile) = self.tiles.get(&new_position) {
                if *tile != TrackTile::Wall {
                    neighbours.push(new_position);
                }
            }
        }
        neighbours
    }
    fn neighbours(&self, position: TrackPosition) -> Vec<TrackPosition> {
        let mut neighbours = Vec::new();
        let x = position.x as i32;
        let y = position.y as i32;
        let directions = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
        for (dx, dy) in directions {
            let new_position = TrackPosition {
                x: (x + dx) as usize,
                y: (y + dy) as usize,
            };
            if let Some(tile) = self.tiles.get(&new_position) {
                if *tile != TrackTile::Wall {
                    neighbours.push(new_position);
                }
            }
        }
        neighbours
    }
    fn find_all_paths(&mut self) -> Vec<Vec<TrackPosition>> {
        let mut possible_paths = Vec::new();
        let mut visited = HashSet::new(); // Track (position, cheated_state)
        let mut queue = VecDeque::new();

        // Start the queue with the starting position and a flag indicating if we've cheated
        queue.push_back((vec![self.start], self.start));

        while let Some((path, position)) = queue.pop_front() {
            // When we reach the end, record the path
            if position == self.end {
                possible_paths.push(path);
            } else {
                let cheated = visited.contains(&(position, true));
                let neighbours = if cheated {
                    self.cheat_neighbours(position)
                } else {
                    self.neighbours(position)
                };
                for neighbour in neighbours {
                    let mut new_path = path.clone();
                    new_path.push(neighbour);
                    let new_cheated =
                        cheated || self.tiles.get(&neighbour) == Some(&TrackTile::Empty);
                    if !visited.contains(&(neighbour, new_cheated)) {
                        visited.insert((neighbour, new_cheated));
                        queue.push_back((new_path, neighbour));
                    }
                }
            }
        }
        possible_paths
    }
}
impl Display for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.tiles.keys().map(|position| position.y).max().unwrap() + 1 {
            for x in 0..self.tiles.keys().map(|position| position.x).max().unwrap() + 1 {
                let position = TrackPosition { x, y };
                let tile = self.tiles.get(&position).unwrap_or(&TrackTile::Wall);
                print!("{}", tile);
            }
            print!("{}", "\n");
        }
        Ok(())
    }
}
impl TryFrom<&str> for Track {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tiles = HashMap::new();
        let mut start = None;
        let mut end = None;
        value.trim().lines().enumerate().for_each(|(y, line)| {
            line.trim().chars().enumerate().for_each(|(x, tile)| {
                let position = TrackPosition { x, y };
                if let Ok(track_tile) = TrackTile::try_from(tile) {
                    tiles.insert(position, track_tile);
                    match track_tile {
                        TrackTile::Start => {
                            start = Some(position);
                        }
                        TrackTile::End => {
                            end = Some(position);
                        }
                        _ => {}
                    }
                }
            });
        });
        Ok(Track {
            tiles,
            start: start.ok_or(())?,
            end: end.ok_or(())?,
        })
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum TrackTile {
    Empty,
    Wall,
    Start,
    End,
    Raced,
}
impl TryFrom<char> for TrackTile {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(TrackTile::Empty),
            '#' => Ok(TrackTile::Wall),
            'S' => Ok(TrackTile::Start),
            'E' => Ok(TrackTile::End),
            _ => Err(()),
        }
    }
}
impl Display for TrackTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TrackTile::Empty => ".".yellow(),
                TrackTile::Wall => "#".red(),
                TrackTile::Start => "S".blue(),
                TrackTile::End => "E".green(),
                TrackTile::Raced => "X".cyan(),
            }
        )
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct TrackPosition {
    x: usize,
    y: usize,
}
