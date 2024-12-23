use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    const REINDEER_MAZE: &str = r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
        "#;
    #[test]
    fn read_maze() {
        let maze = super::Maze::try_from(REINDEER_MAZE).unwrap();
        assert_eq!(maze.tiles.len(), 15 * 15);
        assert_eq!(maze.start, super::MazePosition { x: 1, y: 13 });
        assert_eq!(maze.end, super::MazePosition { x: 13, y: 1 });
    }
    #[test]
    fn walk_maze() {
        let maze = super::Maze::try_from(REINDEER_MAZE).unwrap();
        let shortest_path = maze.find_shortest_path_dfs().unwrap();
        assert_eq!(shortest_path.0.len(), 36);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    reindeer: Reindeer,
    steps: usize,
    turns: usize,
}
struct Maze {
    tiles: HashMap<MazePosition, MazeTile>,
    start: MazePosition,
    end: MazePosition,
}
impl Maze {
    fn find_shortest_path_dfs(&self) -> Option<(Vec<MazePosition>, usize, usize)> {
        let mut shortest_path = None;
        let initial_reindeer = Reindeer {
            position: self.start,
            direction: ReindeerDirection::Right,
        };
        let initial_state = State {
            reindeer: initial_reindeer,
            steps: 0,
            turns: 0,
        };

        self.dfs(
            initial_state,
            &mut shortest_path,
            &mut vec![],
            &mut HashSet::new(),
        );
        shortest_path
    }

    fn dfs(
        &self,
        state: State,
        shortest_path: &mut Option<(Vec<MazePosition>, usize, usize)>,
        current_path: &mut Vec<MazePosition>,
        visited: &mut HashSet<(MazePosition, ReindeerDirection)>,
    ) {
        let current_position = state.reindeer.position;

        // Mark current state as visited
        if !visited.insert((current_position, state.reindeer.direction)) {
            return; // If already visited, backtrack
        }
        current_path.push(current_position);

        if current_position == self.end {
            // Found path to end, check if it's the shortest
            if let Some((_, shortest_steps, shortest_turns)) = shortest_path {
                if state.steps < *shortest_steps
                    || (state.steps == *shortest_steps && state.turns < *shortest_turns)
                {
                    *shortest_path = Some((current_path.clone(), state.steps, state.turns));
                }
            } else {
                // First path to end
                *shortest_path = Some((current_path.clone(), state.steps, state.turns));
            }
            current_path.pop(); // Backtrack
            return;
        }

        let possible_directions = vec![
            ReindeerDirection::Up,
            ReindeerDirection::Down,
            ReindeerDirection::Left,
            ReindeerDirection::Right,
        ];

        for direction in possible_directions {
            let mut new_reindeer = state.reindeer;
            new_reindeer.direction = direction;
            let next_position = new_reindeer.check_next_position();

            if self.check_next_position(next_position, direction) {
                let new_steps = state.steps + 1;
                let new_turns = if direction != state.reindeer.direction {
                    state.turns + 1
                } else {
                    state.turns
                };

                self.dfs(
                    State {
                        reindeer: Reindeer {
                            position: next_position,
                            direction,
                        },
                        steps: new_steps,
                        turns: new_turns,
                    },
                    shortest_path,
                    current_path,
                    visited,
                );
            }
        }

        // Backtrack
        visited.remove(&(current_position, state.reindeer.direction));
        current_path.pop();
    }
    fn check_next_position(&self, position: MazePosition, direction: ReindeerDirection) -> bool {
        let next_position = match direction {
            ReindeerDirection::Up => MazePosition {
                x: position.x,
                y: position.y - 1,
            },
            ReindeerDirection::Down => MazePosition {
                x: position.x,
                y: position.y + 1,
            },
            ReindeerDirection::Left => MazePosition {
                x: position.x - 1,
                y: position.y,
            },
            ReindeerDirection::Right => MazePosition {
                x: position.x + 1,
                y: position.y,
            },
        };
        match self.tiles.get(&next_position) {
            Some(MazeTile::Wall) => false,
            Some(MazeTile::End) => true,
            Some(MazeTile::Empty) => true,
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ReindeerDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Reindeer {
    position: MazePosition,
    direction: ReindeerDirection,
}
impl Reindeer {
    fn check_next_position(&self) -> MazePosition {
        match self.direction {
            ReindeerDirection::Up => MazePosition {
                x: self.position.x,
                y: self.position.y - 1,
            },
            ReindeerDirection::Down => MazePosition {
                x: self.position.x,
                y: self.position.y + 1,
            },
            ReindeerDirection::Left => MazePosition {
                x: self.position.x - 1,
                y: self.position.y,
            },
            ReindeerDirection::Right => MazePosition {
                x: self.position.x + 1,
                y: self.position.y,
            },
        }
    }
    fn walk_once(&mut self) {
        match self.direction {
            ReindeerDirection::Up => self.position.y -= 1,
            ReindeerDirection::Down => self.position.y += 1,
            ReindeerDirection::Left => self.position.x -= 1,
            ReindeerDirection::Right => self.position.x += 1,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MazeTile {
    Wall,
    Empty,
    Start,
    End,
}
impl From<char> for MazeTile {
    fn from(c: char) -> Self {
        match c {
            '#' => MazeTile::Wall,
            '.' => MazeTile::Empty,
            'S' => MazeTile::Start,
            'E' => MazeTile::End,
            _ => panic!("Invalid character"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MazePosition {
    x: i64,
    y: i64,
}
impl TryFrom<&str> for Maze {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tiles = HashMap::new();
        let mut start = None;
        let mut end = None;
        for (y, line) in value.trim().lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                let position = MazePosition {
                    x: x as i64,
                    y: y as i64,
                };
                let tile = MazeTile::from(c);
                match tile {
                    MazeTile::Start => {
                        if start.is_some() {
                            return Err("Multiple start positions");
                        }
                        start = Some(position);
                    }
                    MazeTile::End => {
                        if end.is_some() {
                            return Err("Multiple end positions");
                        }
                        end = Some(position);
                    }
                    _ => {}
                }
                tiles.insert(position, tile);
            }
        }
        let start = start.ok_or("No start position")?;
        let end = end.ok_or("No end position")?;
        Ok(Maze { tiles, start, end })
    }
}
