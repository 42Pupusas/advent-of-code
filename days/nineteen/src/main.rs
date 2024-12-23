use std::fmt::Display;

use colored::Colorize;

fn main() {
    println!("Hello, world!");
    let input_str = include_str!("../input.txt");
    println!("Part one: {}", part_one(input_str));
    println!("Part two: {}", part_two(input_str));
}
fn part_one(input_str: &str) -> usize {
    let input_str = input_str.trim().lines();
    let pattern_line = input_str.clone().next().unwrap();
    let patterns = pattern_line
        .split(",")
        .filter_map(|p| TowelPattern::try_from(p.trim()).ok())
        .collect::<Vec<_>>();
    let targets = input_str
        .skip(1)
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                OnsenTowel::build_towel_return(s, &patterns)
            }
        })
        .collect::<Vec<_>>();
    targets.len()
}
fn part_two(input_str: &str) -> usize {
    let input_str = input_str.trim().lines();
    let pattern_line = input_str.clone().next().unwrap();
    let patterns = pattern_line
        .split(",")
        .filter_map(|p| TowelPattern::try_from(p.trim()).ok())
        .collect::<Vec<_>>();
    let targets = input_str
        .skip(1)
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                let variations = OnsenTowel::count_ways_to_build_towel(s, &patterns);
                Some(variations)
            }
        })
        .sum::<usize>();
    targets
}
#[cfg(test)]
mod tests {
    use super::*;
    const ONSEN_TOWELS: &str = r#"
    r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
        "#;
    #[test]
    fn onsen_patterns() -> Result<(), &'static str> {
        let input_str = ONSEN_TOWELS.trim().lines();
        let pattern_line = input_str.clone().next().ok_or("No pattern line")?;
        let patterns = pattern_line
            .split(",")
            .filter_map(|p| TowelPattern::try_from(p.trim()).ok())
            .collect::<Vec<_>>();
        patterns.iter().for_each(|p| println!("{}", p));
        let targets = input_str
            .skip(1)
            .filter_map(|s| {
                let s = s.trim();
                if s.is_empty() {
                    None
                } else {
                    let variations = OnsenTowel::count_ways_to_build_towel(s, &patterns);
                    Some(variations)
                }
            })
            .sum::<usize>();
        assert_eq!(targets, 16);
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct OnsenTowel {
    pattern: Vec<TowelPattern>,
}
impl OnsenTowel {
    fn count_ways_to_build_towel(target: &str, patterns: &[TowelPattern]) -> usize {
        let target_len = target.len();
        let mut dp = vec![0; target_len + 1];
        dp[0] = 1; // There's one way to build an empty string (no patterns)

        // Iterate through each position in the target string
        for i in 1..=target_len {
            // Check each smaller pattern
            for pattern in patterns {
                let pattern_len = pattern.stripes.len();
                if i >= pattern_len && dp[i - pattern_len] > 0 {
                    let pattern_str = &target[i - pattern_len..i];
                    if let Ok(towel_pattern) = TowelPattern::try_from(pattern_str) {
                        if towel_pattern == *pattern {
                            dp[i] += dp[i - pattern_len]; // Increment ways to build up to i
                        }
                    }
                }
            }
        }

        dp[target_len] // Return the number of ways to build the entire target string
    }
    fn build_towel_return(target: &str, patterns: &[TowelPattern]) -> Option<Self> {
        let target_len = target.len();
        let mut dp = vec![None; target_len + 1];
        dp[0] = Some(vec![]); // Initialize with an empty pattern to start the process

        // Iterate through each position in the target string
        for i in 1..=target_len {
            // Check each smaller pattern
            for pattern in patterns {
                let pattern_len = pattern.stripes.len();
                if i >= pattern_len && dp[i - pattern_len].is_some() {
                    let pattern_str = &target[i - pattern_len..i];
                    if let Ok(towel_pattern) = TowelPattern::try_from(pattern_str) {
                        if towel_pattern == *pattern {
                            if dp[i - pattern_len].is_some() {
                                // If dp[i - pattern_len] is valid, append this pattern to it
                                let mut new_pattern_sequence =
                                    dp[i - pattern_len].as_ref().unwrap().clone();
                                new_pattern_sequence.push(towel_pattern.clone());
                                dp[i] = Some(new_pattern_sequence);
                                break; // No need to check further for this i
                            }
                        }
                    }
                }
            }
        }

        // If dp[target_len] is Some, return the sequence of patterns
        dp[target_len].clone().map(|pattern_sequence| Self {
            pattern: pattern_sequence,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TowelPattern {
    stripes: Vec<TowelStripe>,
}
impl TryFrom<&str> for TowelPattern {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            stripes: value
                .chars()
                .filter_map(|c| TowelStripe::try_from(c).ok())
                .collect(),
        })
    }
}
impl Display for TowelPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.stripes
            .iter()
            .try_for_each(|stripe| write!(f, "{}", stripe))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TowelStripe {
    White,
    Black,
    Blue,
    Red,
    Green,
}
impl TryFrom<char> for TowelStripe {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(TowelStripe::White),
            'b' => Ok(TowelStripe::Black),
            'u' => Ok(TowelStripe::Blue),
            'r' => Ok(TowelStripe::Red),
            'g' => Ok(TowelStripe::Green),
            _ => Err("Invalid character"),
        }
    }
}
impl Display for TowelStripe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TowelStripe::White => write!(f, "{}", "w".white()),
            TowelStripe::Black => write!(f, "{}", "b".cyan()),
            TowelStripe::Blue => write!(f, "{}", "u".blue()),
            TowelStripe::Red => write!(f, "{}", "r".red()),
            TowelStripe::Green => write!(f, "{}", "g".green()),
        }
    }
}
