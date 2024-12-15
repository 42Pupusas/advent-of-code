use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    sync::{Arc, RwLock},
};

fn main() {
    println!("Hello, advent of code day eleven!");
    let input = include_str!("../input.txt");
    println!();
    println!();
    println!("Part one:");
    println!();
    let mut stones = StoneArrangement::from(input);
    println!("{}", stones);
    for _ in 0..25 {
        stones.blink_once();
    }
    println!();
    println!("{}", stones.stones.len());
    println!();
    println!();
    println!("Part two:");
    println!();
    let split_input = input
        .split_whitespace()
        .map(|node| TimeStone {
            engraving: node.chars().collect(),
        })
        .collect::<Vec<TimeStone>>();
    println!();
    println!();
}
#[cfg(test)]
mod tests {
    use crate::{StoneArrangement, TimeStone};

    const TEST_STRING: &str = "0 1 10 99 999";
    const RESULT_STRING: &str = "1 2024 1 0 9 9 2021976";
    #[test]
    fn blink_once() {
        let input = TEST_STRING
            .split_whitespace()
            .map(|x| TimeStone {
                engraving: x.chars().collect(),
            })
            .collect::<Vec<TimeStone>>();
        let stones = StoneArrangement { stones: input };
        println!("{}", stones);
        let blinked_stone = stones.stones.iter().map(|stone| stone.blink()).fold(
            StoneArrangement { stones: vec![] },
            |mut acc, x| {
                acc.stones.extend(x);
                acc
            },
        );
        println!("{}", blinked_stone,);
        assert!(RESULT_STRING == blinked_stone.to_string());
    }

    const TEST_STRING_2: &str = "125 17";
    const FIRST_BLINK: &str = "253000 1 7";
    const SECOND_BLINK: &str = "253 0 2024 14168";
    const THIRD_BLINK: &str = "512072 1 20 24 28676032";
    const FOURTH_BLINK: &str = "512 72 2024 2 0 2 4 2867 6032";
    const FIFTH_BLINK: &str = "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32";
    const SIXTH_BLINK: &str =
        "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2";
    #[test]
    fn blink_six_times() {
        let mut stones = StoneArrangement::from(TEST_STRING_2);
        println!("{}", stones);
        for i in 0..6 {
            stones.blink_once();
            println!("{}", stones);
            match i {
                0 => assert!(FIRST_BLINK == stones.to_string()),
                1 => assert!(SECOND_BLINK == stones.to_string()),
                2 => assert!(THIRD_BLINK == stones.to_string()),
                3 => assert!(FOURTH_BLINK == stones.to_string()),
                4 => assert!(FIFTH_BLINK == stones.to_string()),
                5 => assert!(SIXTH_BLINK == stones.to_string()),
                _ => panic!("Unexpected iteration"),
            }
        }
    }
    #[test]
    fn blink_twenty_five_times() {
        let mut stones = StoneArrangement::from(TEST_STRING_2);
        println!("{}", stones);
        for _ in 0..25 {
            stones.blink_once();
        }
        assert_eq!(stones.stones.len(), 55312);
    }
    #[test]
    fn blink_recursive() {
        let mut stones = StoneArrangement::from(TEST_STRING_2);
        let mut engraving_map = std::collections::HashMap::new();
        stones.blink_recursive(25, &mut engraving_map);
        let total = engraving_map.values().sum::<u64>();
        assert_eq!(total, 55312);
    }
}
struct StoneArrangement {
    stones: Vec<TimeStone>,
}
impl StoneArrangement {
    fn blink_once(&mut self) {
        self.stones = self
            .stones
            .iter()
            .map(|stone| stone.blink())
            .fold(vec![], |mut acc, x| {
                acc.extend(x);
                acc
            });
    }
    fn blink_recursive(&mut self, iterations: u64, engraving_map: &mut HashMap<TimeStone, u64>) {
        engraving_map.insert(self.stones[0].clone(), 1);
        for _ in 0..iterations {
            let new_stones = engraving_map.keys().cloned().collect::<Vec<TimeStone>>();
            new_stones.iter().map(|stone| stone.blink()).for_each(|x| {
                x.iter().for_each(|new_stone| {
                    engraving_map
                        .entry(new_stone.clone())
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                });
            });
        }
    }
}
impl From<&str> for StoneArrangement {
    fn from(item: &str) -> Self {
        StoneArrangement {
            stones: item
                .split_whitespace()
                .map(|x| TimeStone {
                    engraving: x.chars().collect(),
                })
                .collect(),
        }
    }
}
impl Display for TimeStone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.engraving.iter().collect::<String>())
    }
}
impl Display for StoneArrangement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.stones
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct TimeStone {
    engraving: Vec<char>,
}
impl TimeStone {
    fn blink(&self) -> Vec<TimeStone> {
        let engraving_num: u64 = self.into();
        match self.engraving.len() % 2 {
            0 => {
                let (left_engraving, right_engraving) =
                    self.engraving.split_at(self.engraving.len() / 2);
                vec![
                    TimeStone::from(left_engraving),
                    TimeStone::from(right_engraving),
                ]
            }
            _ => {
                if engraving_num == 0 {
                    vec![TimeStone::from('1')]
                } else {
                    let new_engraving_num = engraving_num * 2024;
                    vec![TimeStone::from(new_engraving_num)]
                }
            }
        }
    }
}
impl Into<u64> for &TimeStone {
    fn into(self) -> u64 {
        self.engraving
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }
}
impl Into<u64> for TimeStone {
    fn into(self) -> u64 {
        self.engraving
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }
}
impl From<&[char]> for TimeStone {
    fn from(item: &[char]) -> Self {
        let engraving = item.iter().collect::<String>();
        let trimmed = engraving.trim_start_matches('0');
        if trimmed.is_empty() {
            return TimeStone::from('0');
        }
        TimeStone {
            engraving: trimmed.chars().collect(),
        }
    }
}
impl From<char> for TimeStone {
    fn from(item: char) -> Self {
        TimeStone {
            engraving: vec![item],
        }
    }
}
impl From<u64> for TimeStone {
    fn from(item: u64) -> Self {
        TimeStone {
            engraving: item.to_string().chars().collect(),
        }
    }
}
