use std::fmt::Display;

use colored::Colorize;
pub fn second_part_try_two(input: &str) {
    let chars = input.chars().collect::<Vec<char>>();
    let mut disk_blocks = Vec::new();
    chars.chunks(2).enumerate().for_each(|(position, pair)| {
        if let Some(first) = pair.get(0) {
            let size = first.to_digit(10).expect("Invalid size") as u64;
            let position = position as u64;
            let disk_block = DiskBlock {
                position,
                size,
                file: true,
            };
            disk_blocks.push(disk_block);
        }
        if let Some(second) = pair.get(1) {
            let size = second.to_digit(10).expect("Invalid size") as u64;
            let position = position as u64;
            let disk_block = DiskBlock {
                position,
                size,
                file: false,
            };
            disk_blocks.push(disk_block);
        }
    });
    disk_blocks.sort_by(|a, b| a.position.cmp(&b.position));
    disk_blocks.chunks_mut(2).for_each(|pair| {
        pair.sort_by(|a, b| b.file.cmp(&a.file));
    });

    let mut right_index = disk_blocks.len() as u64;
    let mut left_index = 0;

    while right_index > left_index {
        let (last_file_pos, last_file_block) = disk_blocks
            .iter()
            .enumerate()
            .rev()
            .skip(disk_blocks.len() - right_index as usize)
            .find(|(_, block)| block.file)
            .expect("No file found");
        let last_block_size = last_file_block.size;
        if let Some((empty_pos, empty_block)) = disk_blocks
            .iter()
            .enumerate()
            .take(last_file_pos)
            .find(|(_, block)| !block.file && block.size >= last_file_block.size)
        {
            let empty_block_size = empty_block.size;
            let empty_block_position = empty_block.position;
            if empty_block_size == last_block_size {
                disk_blocks.swap(empty_pos, last_file_pos);
            }
            if empty_block_size > last_block_size {
                // Create a new empty block with the remaining size
                let new_empty_block = DiskBlock {
                    position: empty_block_position, // Adjust the position of the remainder
                    size: empty_block_size - last_block_size,
                    file: false,
                };
                // Update the original empty block to reflect the size of the file block
                let updated_empty_block = DiskBlock {
                    position: last_file_pos as u64,
                    size: last_block_size, // Size of the file block
                    file: false,
                };
                // Swap the file block with the original empty block
                disk_blocks.swap(empty_pos, last_file_pos);
                // Update the swapped empty block's size
                disk_blocks[last_file_pos] = updated_empty_block;
                // Insert the new empty block after the swapped file block
                disk_blocks.insert(empty_pos + 1, new_empty_block);
            }
            if left_index < empty_block_position {
                left_index = empty_block_position;
            }
            if left_index == 0 {
                left_index = empty_block_position;
            }
        } else {
            right_index -= last_block_size;
            continue;
        }
        right_index -= last_block_size;
    }
    let mut raw_mem = Vec::new();
    disk_blocks.iter().for_each(|block| {
        for _ in 0..block.size {
            if block.file {
                raw_mem.push(block.position);
            } else {
                raw_mem.push(0);
            }
        }
    });
    let total = raw_mem
        .iter()
        .enumerate()
        .fold(0, |mut acc, (index, mem_char)| {
            print_different_color_for_digits(*mem_char);
            acc += mem_char * index as u64;
            acc
        });
    println!("Total: {}", total);
}

fn print_different_color_for_digits(digit: u64) {
    let mod_2 = digit % 2;
    let mod_3 = digit % 3;
    let mod_5 = digit % 5;
    let mod_7 = digit % 7;
    if digit == 0 {
        print!("{}", ".".to_string().white());
    } else if mod_2 == 0 {
        print!("{}", digit.to_string().green());
    } else if mod_3 == 0 {
        print!("{}", digit.to_string().blue());
    } else if mod_5 == 0 {
        print!("{}", digit.to_string().yellow());
    } else if mod_7 == 0 {
        print!("{}", digit.to_string().red());
    } else {
        print!("{}", digit.to_string().purple());
    }
}

#[cfg(test)]
mod tests {
    use super::DiskBlock;

    const TEST_DISK_MAP: &str = "2333133121414131402";
    const TEST_RESULT: &str = "00...111...2...333.44.5555.6666.777.888899";
    #[test]
    fn second_disk_read() {
        let chars = TEST_DISK_MAP.chars().collect::<Vec<char>>();
        let mut disk_blocks = Vec::new();
        chars.chunks(2).enumerate().for_each(|(position, pair)| {
            if let Some(first) = pair.get(0) {
                let size = first.to_digit(10).expect("Invalid size") as u64;
                let position = position.to_string().parse().expect("Invalid position");
                let disk_block = DiskBlock {
                    position,
                    size,
                    file: true,
                };
                disk_blocks.push(disk_block);
            }
            if let Some(second) = pair.get(1) {
                let size = second.to_digit(10).expect("Invalid size") as u64;
                let position = position.to_string().parse().expect("Invalid position");
                let disk_block = DiskBlock {
                    position,
                    size,
                    file: false,
                };
                disk_blocks.push(disk_block);
            }
        });
        disk_blocks.sort_by(|a, b| a.position.cmp(&b.position));
        disk_blocks.chunks_mut(2).for_each(|pair| {
            pair.sort_by(|a, b| b.file.cmp(&a.file));
        });

        let mut right_index = disk_blocks.len() as u64;
        let mut left_index = 0;

        while right_index > left_index {
            let (last_file_pos, last_file_block) = disk_blocks
                .iter()
                .enumerate()
                .rev()
                .skip(disk_blocks.len() - right_index as usize)
                .find(|(_, block)| block.file)
                .expect("No file found");
            let last_block_size = last_file_block.size;
            if let Some((empty_pos, empty_block)) = disk_blocks
                .iter()
                .enumerate()
                .take(last_file_pos)
                .find(|(_, block)| !block.file && block.size >= last_file_block.size)
            {
                let empty_block_size = empty_block.size;
                let empty_block_position = empty_block.position;
                if empty_block_size == last_block_size {
                    disk_blocks.swap(empty_pos, last_file_pos);
                }
                if empty_block_size > last_block_size {
                    // Create a new empty block with the remaining size
                    let new_empty_block = DiskBlock {
                        position: empty_block_position + last_block_size, // Adjust the position of the remainder
                        size: empty_block_size - last_block_size,
                        file: false,
                    };
                    // Update the original empty block to reflect the size of the file block
                    let updated_empty_block = DiskBlock {
                        position: empty_block_position,
                        size: last_block_size, // Size of the file block
                        file: false,
                    };
                    // Swap the file block with the original empty block
                    disk_blocks.swap(empty_pos, last_file_pos);
                    // Update the swapped empty block's size
                    disk_blocks[last_file_pos] = updated_empty_block;
                    // Insert the new empty block after the swapped file block
                    disk_blocks.insert(empty_pos + 1, new_empty_block);
                }
                if left_index < empty_block_position {
                    left_index = empty_block_position;
                }
                if left_index == 0 {
                    left_index = empty_block_position;
                }
            } else {
                right_index -= last_block_size;
                continue;
            }
            right_index -= last_block_size;
        }
        let mut raw_mem = Vec::new();
        disk_blocks.iter().for_each(|block| {
            for _ in 0..block.size {
                if block.file {
                    raw_mem.push(block.position);
                } else {
                    raw_mem.push(0);
                }
            }
        });
        let total = raw_mem
            .iter()
            .enumerate()
            .fold(0, |mut acc, (index, mem_char)| {
                acc += mem_char * index as u64;
                acc
            });
        println!("Total: {}", total);
        assert_eq!(2858, total);
    }
}
#[derive(Debug)]
pub struct DiskBlock {
    pub position: u64,
    pub size: u64,
    pub file: bool,
}
impl Display for DiskBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.size {
            if self.file {
                write!(f, "{}", self.position.to_string().blue())?;
            } else {
                write!(f, "{}", ".".yellow())?;
            }
        }
        Ok(())
    }
}
