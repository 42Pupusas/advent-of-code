use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use colored::Colorize;

fn main() -> Result<(), &'static str> {
    println!("Hello, advent of code eighteen!");
    let input_str = include_str!("../input.txt");
    // part_one(input_str)?;
    part_two(input_str)?;
    Ok(())
}
fn part_two(input: &str) -> Result<(), &'static str> {
    let memory_space = MemorySpace::new(70);
    let corrupted_memory = CorruptedMemory::try_from(input)?;
    let mut nanoseconds = 2870;
    loop {
        let mut new_space = memory_space.clone();
        new_space.corrupt_memory(nanoseconds, &corrupted_memory);
        if let Some(shortest) = new_space.bfs() {
            new_space.safe_memory(&shortest);
            print!(
                "At {} Shortest path is {}\r",
                nanoseconds,
                shortest.len() - 1
            );
            nanoseconds += 1;
            println!("{}", new_space);
        } else {
            println!("{}", new_space);
            break;
        }
    }
    println!();
    println!("Nanoseconds: {}", nanoseconds);
    println!(
        "Path was blocked by {}",
        corrupted_memory
            .corrupted_positions
            .get((nanoseconds - 1) as usize)
            .unwrap()
    );

    Ok(())
}

fn part_one(input: &str) -> Result<(), &'static str> {
    let mut memory_space = MemorySpace::new(70);
    let corrupted_memory = CorruptedMemory::try_from(input)?;

    memory_space.corrupt_memory(1024, &corrupted_memory);
    println!("{}", memory_space);
    let shortest = memory_space.bfs().ok_or("No path found")?;
    memory_space.safe_memory(&shortest);
    println!("{}", memory_space);
    println!("Shortest path is {:?}", shortest.len() - 1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const CORRUPTED_MEMORY_TEST: &str = r#"
    5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#;
    #[test]
    fn byte_map() -> Result<(), &'static str> {
        let mut map = MemorySpace::new(6);
        println!("{}", map);
        let corrupted_memory = CorruptedMemory::try_from(CORRUPTED_MEMORY_TEST)?;
        map.corrupt_memory(12, &corrupted_memory);
        println!("{}", map);
        let shortest = map.bfs().ok_or("No path found")?;
        map.safe_memory(&shortest);
        println!("{}", map);
        println!("Shortes path is {:?}", shortest.len() - 1);
        Ok(())
    }
}
#[derive(Debug, Clone)]
struct MemorySpace {
    size: isize,
    memory: HashMap<MemoryPosition, MemoryValue>,
}
impl MemorySpace {
    fn new(size: isize) -> MemorySpace {
        let mut memory = HashMap::new();
        for x in 0..size + 1 {
            for y in 0..size + 1 {
                memory.insert(MemoryPosition { x, y }, MemoryValue::Safe);
            }
        }
        MemorySpace { size, memory }
    }
    fn is_safe(&self, position: MemoryPosition) -> bool {
        position.x >= 0
            && position.y >= 0
            && position.x <= self.size
            && position.y <= self.size
            && self.memory.get(&position) == Some(&MemoryValue::Safe)
    }
    fn bfs(&mut self) -> Option<Vec<MemoryPosition>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let start_position = self.start_position();
        let end_position = self.end_position();

        // Queue stores the position and the path taken to reach it
        queue.push_back((start_position, vec![start_position]));
        visited.insert(start_position);

        // BFS loop
        while let Some((current_pos, path)) = queue.pop_front() {
            // If we reach the end position, return the path
            if current_pos == end_position {
                return Some(path);
            }
            // Explore the four possible directions
            let directions = [
                MemoryPosition {
                    x: current_pos.x + 1,
                    y: current_pos.y,
                }, // Down
                MemoryPosition {
                    x: current_pos.x - 1,
                    y: current_pos.y,
                }, // Up
                MemoryPosition {
                    x: current_pos.x,
                    y: current_pos.y + 1,
                }, // Right
                MemoryPosition {
                    x: current_pos.x,
                    y: current_pos.y - 1,
                }, // Left
            ];
            for direction in directions.iter() {
                if self.is_safe(*direction) && !visited.contains(direction) {
                    visited.insert(*direction);
                    let mut new_path = path.clone();
                    new_path.push(*direction);
                    queue.push_back((*direction, new_path));
                }
            }
        }

        // If we exit the loop without finding a path, return None
        None
    }
    fn safe_memory(&mut self, safe_memory: &[MemoryPosition]) {
        safe_memory.iter().for_each(|position| {
            self.memory.insert(*position, MemoryValue::Walked);
        });
    }
    fn corrupt_memory(&mut self, size: isize, corrupted_memory: &CorruptedMemory) {
        corrupted_memory
            .take_bytes(size)
            .iter()
            .for_each(|position| {
                self.memory.insert(*position, MemoryValue::Corrupted);
            });
    }
    fn start_position(&self) -> MemoryPosition {
        MemoryPosition { x: 0, y: 0 }
    }
    fn end_position(&self) -> MemoryPosition {
        MemoryPosition {
            x: self.size,
            y: self.size,
        }
    }
}
impl Display for MemorySpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size + 1 {
            for x in 0..self.size + 1 {
                let value = self.memory.get(&MemoryPosition { x, y }).unwrap();
                write!(f, "{}", value)?;
            }
            write!(f, "\n")?;
        }
        writeln!(f)
    }
}
struct CorruptedMemory {
    corrupted_positions: Vec<MemoryPosition>,
}
impl CorruptedMemory {
    fn take_bytes(&self, size: isize) -> &[MemoryPosition] {
        &self.corrupted_positions[..size as usize]
    }
}
impl TryFrom<&str> for CorruptedMemory {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let corrupted_positions = value
            .trim()
            .lines()
            .filter_map(|line| MemoryPosition::try_from(line).ok())
            .collect::<Vec<MemoryPosition>>();
        Ok(CorruptedMemory {
            corrupted_positions,
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MemoryValue {
    Safe,
    Corrupted,
    Walked,
}
impl Display for MemoryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryValue::Safe => write!(f, "{}", ".".bright_green()),
            MemoryValue::Corrupted => write!(f, "{}", "#".bright_red()),
            MemoryValue::Walked => write!(f, "{}", "O".bright_blue()),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MemoryPosition {
    x: isize,
    y: isize,
}
impl TryFrom<&str> for MemoryPosition {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x_str, y_str) = value
            .trim()
            .split_once(',')
            .ok_or("Invalid memory position")?;
        let x = x_str.trim().parse().map_err(|_| "Invalid x position")?;
        let y = y_str.trim().parse().map_err(|_| "Invalid y position")?;
        Ok(MemoryPosition { x, y })
    }
}
impl Display for MemoryPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
