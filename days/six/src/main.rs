use std::{collections::HashMap, fmt::Display, usize};
#[tokio::main]
async fn main() {
    part_one();
    part_two().await;
}
fn part_one() {
    println!("Hello, advent of code day six!");
    let input = include_str!("../input.txt");
    let mut guard_map = GuardMap::from_string(input);
    guard_map.move_guard_until_he_leaves_or_loops();
    guard_map.print_map();
    println!();
    println!("Visited locations: {}", guard_map.count_visited_locations());
}

async fn part_two() {
    let input = include_str!("../input.txt");
    let guard_map = GuardMap::from_string(input);
    let mut variations: Vec<GuardMap> = Vec::new();
    let mut positions_to_check: Vec<MapPosition> = Vec::new();
    let mut walked_map = guard_map.clone();
    walked_map.move_guard_until_he_leaves_or_loops();
    walked_map.map.iter().for_each(|(pos, loc)| {
        if let MapLocation::Path(Some(_), _) = loc {
            positions_to_check.push(pos.clone());
        }
    });
    for (num, pos) in positions_to_check.iter().enumerate() {
        let mut new_map = guard_map.clone();
        new_map.add_obstruction(&pos);
        new_map.id = num;
        variations.push(new_map);
    }
    println!("Variations: {}", variations.len());
    let mut tasks = vec![];
    for map in variations.iter_mut() {
        let mut map_clone = map.clone();
        let task = tokio::task::spawn(async move {
            let guard_was_trapped = map_clone.move_guard_until_he_leaves_or_loops();
            if guard_was_trapped {
                1
            } else {
                0
            }
        });
        tasks.push(task);
    }
    println!("Running {} tasks...", tasks.len());
    let mut trapped_counter = 0;
    for task in tasks {
        let result = task.await.unwrap();
        trapped_counter += result;
    }
    println!("Trapped guards: {}", trapped_counter);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Guard {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum VisitedLocation {
    MovingVertical,
    MovingHorizontal,
    Corner,
}
impl Guard {
    pub fn rotate_right(&self) -> Self {
        match self {
            Guard::Up => Guard::Right,
            Guard::Right => Guard::Down,
            Guard::Down => Guard::Left,
            Guard::Left => Guard::Up,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MapPosition {
    x: usize,
    y: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MapLocation {
    Path(Option<VisitedLocation>, Option<Guard>),
    Obstacle,
}
impl MapPosition {
    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_y(&self) -> usize {
        self.y
    }
}
impl Display for MapLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapLocation::Path(Some(direction_passed), _) => match direction_passed {
                VisitedLocation::MovingVertical => write!(f, "\x1b[33m{}\x1b[0m", "|"),
                VisitedLocation::MovingHorizontal => write!(f, "\x1b[33m{}\x1b[0m", "-"),
                VisitedLocation::Corner => write!(f, "\x1b[33m{}\x1b[0m", "+"),
            },
            MapLocation::Path(_, Some(guard)) => match guard {
                Guard::Up => write!(f, "\x1b[33m{}\x1b[0m", "^"),
                Guard::Down => write!(f, "\x1b[33m{}\x1b[0m", "v"),
                Guard::Left => write!(f, "\x1b[33m{}\x1b[0m", "<"),
                Guard::Right => write!(f, "\x1b[33m{}\x1b[0m", ">"),
            },
            MapLocation::Path(None, _) => write!(f, "\x1b[32m{}\x1b[0m", "."),
            MapLocation::Obstacle => write!(f, "\x1b[31m{}\x1b[0m", "#"),
        }
    }
}
impl MapLocation {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => MapLocation::Path(None, None),
            '#' => MapLocation::Obstacle,
            '^' => MapLocation::Path(None, Some(Guard::Up)),
            'v' => MapLocation::Path(None, Some(Guard::Down)),
            _ => panic!("Invalid character"),
        }
    }
}
#[derive(Debug, Clone)]
struct GuardMap {
    id: usize,
    map: HashMap<MapPosition, MapLocation>,
    last_guard_signal: Option<(MapPosition, Guard)>,
}
impl GuardMap {
    pub fn longest_x(&self) -> usize {
        self.map.iter().map(|(pos, _)| pos.get_x()).max().unwrap() + 1
    }
    pub fn longest_y(&self) -> usize {
        self.map.iter().map(|(pos, _)| pos.get_y()).max().unwrap() + 1
    }
    pub fn from_string(input: &str) -> Self {
        let mut map: HashMap<MapPosition, MapLocation> = HashMap::new();
        input.trim().split('\n').enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let location = MapLocation::from_char(c);
                let position = MapPosition { x, y };
                map.insert(position, location);
            });
        });
        let last_guard_signal = map.iter().find_map(|(position, location)| {
            if let MapLocation::Path(_, Some(guard)) = location {
                Some((position.clone(), guard.clone()))
            } else {
                None
            }
        });
        GuardMap {
            id: 0,
            map,
            last_guard_signal,
        }
    }
    pub fn move_guard(&mut self) -> (MapPosition, MapLocation) {
        let (guard_pos, guard) = self.last_guard_signal.clone().unwrap();
        let overflows = |x: usize, y: usize| (x as i32 - 1) < 0 || (y as i32 - 1) < 0;
        if overflows(guard_pos.get_x(), guard_pos.get_y()) {
            self.last_guard_signal = None;
            return (guard_pos, MapLocation::Path(None, None));
        }

        // Calculate the new position based on the current guard direction
        let new_guard = match guard {
            Guard::Up => MapPosition {
                x: guard_pos.get_x(),
                y: guard_pos.get_y() - 1,
            },
            Guard::Down => MapPosition {
                x: guard_pos.get_x(),
                y: guard_pos.get_y() + 1,
            },
            Guard::Left => MapPosition {
                x: guard_pos.get_x() - 1,
                y: guard_pos.get_y(),
            },
            Guard::Right => MapPosition {
                x: guard_pos.get_x() + 1,
                y: guard_pos.get_y(),
            },
        };

        if new_guard.get_x() >= self.longest_x() || new_guard.get_y() >= self.longest_y() {
            self.last_guard_signal = None;
            return (guard_pos, MapLocation::Path(None, None));
        }
        // Check if the new position is a valid path
        match self.map.get_mut(&new_guard) {
            Some(new_location) => {
                // If the new location is a path, update the current guard position
                // check if the new location ha already been visited and compare the direction of
                // the guard with the direction of the visited location
                // if horizontal and vertical meet, turn the location into a corner.
                match new_location {
                    MapLocation::Obstacle => {
                        // If the new location is an obstacle, change the direction of the guard
                        let new_guard = guard.rotate_right();
                        self.last_guard_signal = Some((guard_pos.clone(), new_guard));
                        // clone old position to return it because we didnt move
                        //
                        // update position with a corner
                        if let Some(location) = self.map.get_mut(&guard_pos) {
                            *location = MapLocation::Path(
                                Some(VisitedLocation::Corner),
                                Some(guard.clone()),
                            );
                        }
                        return (
                            guard_pos,
                            MapLocation::Path(Some(VisitedLocation::Corner), Some(guard)),
                        );
                    }
                    _ => {
                        *new_location = MapLocation::Path(
                            Some(match guard {
                                Guard::Up | Guard::Down => VisitedLocation::MovingVertical,
                                Guard::Left | Guard::Right => VisitedLocation::MovingHorizontal,
                            }),
                            Some(guard.clone()),
                        );
                    }
                }
                // Update last_guard_signal to reflect the new position of the guard
                self.last_guard_signal = Some((new_guard, guard));
            }
            _ => panic!("Invalid move"), // If the new location is not valid, panic
        }
        // return the last location visited
        (guard_pos.clone(), self.map.get(&guard_pos).unwrap().clone())
    }
    pub fn move_guard_until_he_leaves_or_loops(&mut self) -> bool {
        let mut corners: HashMap<MapPosition, HashMap<Guard, usize>> = HashMap::new();
        let mut guard_was_trapped = false;
        loop {
            println!("Looping map: {}", self.id);
            let (last_pos, last_location) = self.move_guard();
            if let MapLocation::Path(Some(VisitedLocation::Corner), _) = last_location {
                let entry = corners.entry(last_pos).or_insert_with(HashMap::new);
                let current_guard_direction = self.last_guard_signal.clone().unwrap().1;
                let counter = entry.entry(current_guard_direction).or_insert(0);
                *counter += 1;
                if *counter / 2 >= 2 {
                    guard_was_trapped = true;
                    break;
                }
            }
            if self.last_guard_signal.is_none() {
                break;
            }
        }
        guard_was_trapped
    }
    pub fn add_obstruction(&mut self, pos: &MapPosition) {
        if let Some(location) = self.map.get_mut(pos) {
            *location = MapLocation::Obstacle;
        }
    }
    pub fn print_map(&self) {
        for y in 0..self.longest_y() {
            for x in 0..self.longest_x() {
                let location = self.map.get(&MapPosition { x, y }).unwrap();
                print!("{}", location);
            }
            println!();
        }
    }
    pub fn count_visited_locations(&self) -> usize {
        self.map
            .iter()
            .filter(|(_, location)| {
                if let MapLocation::Path(Some(_), _) = location {
                    true
                } else {
                    false
                }
            })
            .count()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STRING: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        "#;
    #[test]
    fn read_guard_map() {
        let guard_map = GuardMap::from_string(TEST_STRING);
        guard_map.print_map();
        println!();
        println!("{:?}", guard_map.last_guard_signal);
    }
    #[test]
    fn move_guard() {
        let mut guard_map = GuardMap::from_string(TEST_STRING);
        loop {
            guard_map.move_guard();
            guard_map.print_map();
            println!();
            if guard_map.last_guard_signal.is_none() {
                break;
            }
        }
        assert_eq!(guard_map.count_visited_locations(), 41);
    }
    #[test]
    fn add_obstruction() {
        let mut guard_map = GuardMap::from_string(TEST_STRING);
        guard_map.add_obstruction(&MapPosition { x: 3, y: 6 });
        guard_map.print_map();
        let guard_was_trapped = guard_map.move_guard_until_he_leaves_or_loops();
        guard_map.print_map();
        assert!(guard_was_trapped);
    }
    #[test]
    fn make_map_variations() {
        // I wanto check the map for every open space
        // i want to creat a copy of the map, with each open space repaced by an Obstacle
        // only one replacement per copy
        // i want to collect the copies

        let guard_map = GuardMap::from_string(TEST_STRING);
        let mut variations: Vec<GuardMap> = Vec::new();
        let mut open_spaces = Vec::new();
        for (pos, loc) in guard_map.map.iter() {
            if let MapLocation::Path(_, _) = loc {
                open_spaces.push(pos);
            }
        }
        for pos in open_spaces {
            let mut new_map = guard_map.clone();
            new_map.add_obstruction(pos);
            variations.push(new_map);
        }
        for map in variations.iter() {
            map.print_map();
            println!();
        }
    }
    #[test]
    fn count_looped_guards() {
        let guard_map = GuardMap::from_string(TEST_STRING);
        let mut variations: Vec<GuardMap> = Vec::new();
        let mut positions_to_check = Vec::new();
        let mut trapped_counter = 0;
        let mut walked_map = guard_map.clone();
        walked_map.move_guard_until_he_leaves_or_loops();
        walked_map.map.iter().for_each(|(pos, loc)| {
            if let MapLocation::Path(Some(_), _) = loc {
                positions_to_check.push(pos);
            }
        });
        walked_map.print_map();

        for pos in positions_to_check {
            let mut new_map = guard_map.clone();
            new_map.add_obstruction(pos);
            variations.push(new_map);
        }
        println!("Variations: {}", variations.len());
        for map in variations.iter_mut() {
            let guard_was_trapped = map.move_guard_until_he_leaves_or_loops();
            if guard_was_trapped {
                trapped_counter += 1;
            }
        }
        println!("Trapped guards: {}", trapped_counter);
        assert_eq!(trapped_counter, 6);
    }
}
