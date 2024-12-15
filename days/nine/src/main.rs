use colored::Colorize;
use second_try::second_part_try_two;
use std::{collections::HashSet, fmt::Display};
mod second_try;

fn main() {
    println!("Hello, advent of code day nine!");
    let now = std::time::Instant::now();
    let input_str = std::fs::read("days/nine/input.txt")
        .expect("file not found")
        .into_iter()
        .map(|byte| byte as char)
        .collect::<String>().trim().to_string();
    println!("Time to read file: {:?}µs", now.elapsed());
    println!();
    //println!("Part One");
    //part_one(&input_str);
    println!();
    println!("Part Two");
    second_part_try_two(&input_str);
    println!("Time to complete part two: {:?}µs", now.elapsed());
    println!();
}

fn part_one(disk_map: &str) {
    println!();
    println!("Part One");
    println!();
    let memory_manager = MemoryManager::from(disk_map);
    let mut memory_block = memory_manager.memory_blocks();
    // println!("{}", memory_manager);
    // println!();
    memory_block.swap_memory();
    println!("{}", memory_block);
    println!();
    let checksum = memory_block.compute_checksum();
    println!("Checksum: {}", checksum);
}
#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use crate::{MemoryLayout, MemoryManager};

    const TEST_DISK_MAP: &str = "2333133121414131402";
    const TEST_RESULT: &str = "00...111...2...333.44.5555.6666.777.888899";
    #[test]
    fn read_disk_map() {
        let memory_manager = MemoryManager::from(TEST_DISK_MAP);
        let memory_blocks = memory_manager.memory_blocks();

        println!();
        println!("Compact Memory");
        println!();
        println!("{}", TEST_DISK_MAP);
        println!();
        println!();
        println!("Memory Blocks");
        println!();
        println!("{}", memory_blocks);
        println!();
        let mem_string: String = memory_blocks.into();
        assert_eq!(&mem_string, TEST_RESULT);
    }
    const SECOND_TEST_RESULT: &str = "0099811188827773336446555566..............";
    #[test]
    fn swap_memory() {
        let memory_manager = MemoryManager::from(TEST_DISK_MAP);
        let mut memory_blocks = memory_manager.memory_blocks();
        println!("{}", memory_blocks);
        memory_blocks.swap_memory();
        println!();
        let mem_string: String = memory_blocks.into();
        println!("{}", mem_string);
        assert_eq!(&mem_string, SECOND_TEST_RESULT);
    }
    #[test]
    fn compute_checksum() {
        let memory_manager = MemoryManager::from(TEST_DISK_MAP);
        let mut memory_blocks = memory_manager.memory_blocks();
        println!("{}", memory_blocks);
        memory_blocks.swap_memory();
        println!();
        let checksum = memory_blocks.compute_checksum();
        println!("Checksum: {}", checksum);
        assert_eq!(checksum, 1928);
    }
    #[test]
    fn swap_files() {
        let memory_manager = MemoryManager::from(TEST_DISK_MAP);
        println!();
        println!("Memory Layout");
        println!();
        println!("{}", memory_manager.memory_blocks());
        let mut swap_files = memory_manager.memory_layout.iter().collect::<Vec<_>>();
        swap_files.sort_by(|a, b| a.id().cmp(&b.id()));
        swap_files.chunks_mut(2).for_each(|chunk| {
            chunk.sort_by(|a, b| a.order().cmp(&b.order()));
        });

        let left_index = 0;
        let mut right_index = 0;

        loop {
            let last_file_block_position = swap_files
                .iter()
                .skip(right_index)
                .rposition(|block| matches!(block, MemoryLayout::FileBlock(_)))
                .unwrap();
            right_index = last_file_block_position;
            let last_file_block = swap_files[right_index];
            let mut swappable = vec![];
            while let Some(next_last_file_block) = swap_files.get(right_index) {
                if &last_file_block == next_last_file_block {
                    swappable.push(right_index);
                    if right_index > 0 {
                        right_index -= 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            let next_empty_block_that_fits = swap_files.iter().skip(left_index).position(|block| {
                matches!(block, MemoryLayout::EmptyBlock(_))
                    && block.size() >= swappable.len() as u64
            });
            match next_empty_block_that_fits {
                Some(empty_block_position) => {
                    if let Some(empty_block) = swap_files.get_mut(empty_block_position) {
                        if let MemoryLayout::EmptyBlock(block) = empty_block.borrow_mut() {}
                    }
                }
                None => {
                    right_index += 1;
                    continue;
                }
            }
        }
    }
}
#[derive(Clone)]
pub struct RamMemory {
    memory: Vec<MemChar>,
}
impl RamMemory {
    fn swap_memory(&mut self) {
        let mut left_index = 0;
        let mut right_index = self.memory.len() - 1;
        while left_index < right_index {
            if let MemChar::Empty = self.memory[left_index] {
                while left_index < right_index && self.memory[right_index] == MemChar::Empty {
                    right_index -= 1;
                }
                if left_index < right_index {
                    self.memory.swap(left_index, right_index);
                }
            }
            left_index += 1;
        }
    }

    fn compute_checksum(&self) -> u64 {
        self.memory
            .iter()
            .filter_map(|mem_char| match mem_char {
                MemChar::File(mem_id) => Some(mem_id),
                _ => None,
            })
            .enumerate()
            .fold(0 as u64, |mut acc, (position, mem_char)| {
                acc += position as u64 * mem_char;
                acc
            })
    }
}
impl Into<String> for RamMemory {
    fn into(self) -> String {
        self.memory.iter().fold(String::new(), |mut acc, mem_char| {
            acc.push_str(&mem_char.to_string());
            acc
        })
    }
}
impl Display for RamMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.memory.iter().for_each(|mem_char| {
            let _ = write!(f, "{}", mem_char);
        });
        writeln!(f, "")
    }
}
#[derive(Eq, PartialEq, Hash, Clone)]
enum MemChar {
    Empty,
    File(u64),
}
impl Display for MemChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemChar::Empty => write!(f, "{}", ".".yellow()),
            MemChar::File(file_char) => write!(f, "{}", file_char.to_string().green()),
        }
    }
}
#[derive(Eq, PartialEq, Hash, Debug)]
struct MemoryBlock {
    memory_id: u64,
    block_size: u64,
}
#[derive(Eq, PartialEq, Hash, Debug)]
enum MemoryLayout {
    FileBlock(MemoryBlock),
    EmptyBlock(MemoryBlock),
}
impl MemoryLayout {
    fn block(&self) -> Vec<MemChar> {
        match self {
            MemoryLayout::FileBlock(block) => {
                vec![MemChar::File(block.memory_id); block.block_size.try_into().unwrap()]
            }
            MemoryLayout::EmptyBlock(block) => {
                vec![MemChar::Empty; block.block_size.try_into().unwrap()]
            }
        }
    }
    fn id(&self) -> u64 {
        match self {
            MemoryLayout::FileBlock(block) => block.memory_id,
            MemoryLayout::EmptyBlock(block) => block.memory_id,
        }
    }
    fn order(&self) -> u64 {
        match self {
            MemoryLayout::FileBlock(_) => 0,
            MemoryLayout::EmptyBlock(_) => 1,
        }
    }
    fn size(&self) -> u64 {
        match self {
            MemoryLayout::FileBlock(block) => block.block_size,
            MemoryLayout::EmptyBlock(block) => block.block_size,
        }
    }
}
struct MemoryManager {
    memory_layout: HashSet<MemoryLayout>,
}
impl MemoryManager {
    fn memory_blocks(&self) -> RamMemory {
        let mut file_blocks = self
            .memory_layout
            .iter()
            .filter_map(|block| match block {
                MemoryLayout::FileBlock(_) => Some(block),
                _ => None,
            })
            .collect::<Vec<_>>();
        let mut raw_mem = RamMemory { memory: vec![] };
        file_blocks.sort_by(|a, b| a.id().cmp(&b.id()));
        file_blocks.iter().for_each(|file_block| {
            raw_mem.memory.extend(file_block.block());
            if let Some(empty_block) = self
                .memory_layout
                .iter()
                .find(|block| block.id() == file_block.id() && block != file_block)
            {
                raw_mem.memory.extend(empty_block.block());
            }
        });
        raw_mem
    }
}
impl From<&str> for MemoryManager {
    fn from(s: &str) -> Self {
        let chars = s.chars().collect::<Vec<char>>();
        let mut memory_id = 0;
        let memory_layout = chars
            .iter()
            .enumerate()
            .fold(HashSet::new(), |mut acc, (i, c)| {
                if *c == '\n' {
                    return acc;
                }
                if let Ok(block_size) = c.to_string().parse::<u64>() {
                    if i % 2 == 0 {
                        acc.insert(MemoryLayout::FileBlock(MemoryBlock {
                            memory_id,
                            block_size,
                        }));
                    } else {
                        acc.insert(MemoryLayout::EmptyBlock(MemoryBlock {
                            memory_id,
                            block_size,
                        }));
                        memory_id += 1;
                    }
                }
                acc
            });
        MemoryManager { memory_layout }
    }
}
