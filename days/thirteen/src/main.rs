use std::collections::HashSet;

fn main() {
    println!("Hello, advent of code day thirteen!");
    let claw_arcade: ClawArcade = include_str!("../input.txt").try_into().unwrap();
    let total = claw_arcade
        .games
        .iter()
        .filter_map(|game| {
            let (min_a, min_b) = game.token_cost()?;
            Some(min_a + min_b)
        })
        .fold(0, |acc, cost| acc + cost);
    println!();
    println!("Total tokens {}", total);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    const CLAW_INSTRUCTIONS: &str = r#"
    Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;
    #[test]
    fn claw_instructions() {
        let claw_arcade: ClawArcade = CLAW_INSTRUCTIONS.try_into().unwrap();
        println!();
        println!();
        assert_eq!(claw_arcade.games.len(), 4);
    }
    #[test]
    fn claw_presses() {
        println!();
        println!();
        let claw_arcade: ClawArcade = CLAW_INSTRUCTIONS.try_into().unwrap();
        assert_eq!(claw_arcade.games.len(), 4);
        let total = claw_arcade
            .games
            .iter()
            .filter_map(|game| {
                let (min_a, min_b) = game.token_cost()?;
                Some(min_a + min_b)
            })
            .fold(0, |acc, cost| acc + cost);
        println!();
        println!("Total tokens {}", total);
        println!();
        assert_eq!(total, 480);
    }
    const EVIL_CLAW_INSTRUCTIONS: &str = r#"
    Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=10000000012748, Y=10000000012176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=10000000007870, Y=10000000006450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=10000000018641, Y=10000000010279
"#;
    #[test]
    fn evil_presses() {
        println!();
        println!();
        let claw_arcade: ClawArcade = EVIL_CLAW_INSTRUCTIONS.try_into().unwrap();
        assert_eq!(claw_arcade.games.len(), 4);
        let total = claw_arcade
            .games
            .iter()
            .filter_map(|game| {
                let (min_a, min_b) = game.token_cost()?;
                Some(min_a + min_b)
            })
            .fold(0, |acc, cost| acc + cost);
        println!("{}", total);
        assert_eq!(total, 480);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct ClawArcade {
    games: HashSet<ClawGame>,
}
impl TryFrom<&str> for ClawArcade {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            games: value
                .trim()
                .lines()
                .collect::<Vec<_>>()
                .chunks(4)
                .filter_map(|chunk| {
                    chunk.iter().for_each(|line| {
                        println!("{}", line);
                    });
                    let claw_game = ClawGame::try_from(chunk).ok()?;
                    Some(claw_game)
                })
                .collect::<HashSet<_>>(),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ClawGame {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}
impl ClawGame {
    fn evil_button_presses(&self) -> Option<(u64, u64)> {
        let mut min_presses = None;
        for a in 0..=100 {
            for b in 0..=100 {
                let x = self.button_a.x_plus * a + self.button_b.x_plus * b;
                let y = self.button_a.y_plus * a + self.button_b.y_plus * b;
                if x == self.prize.x && y == self.prize.y {
                    min_presses = Some((a, b));
                    break;
                }
            }
        }
        min_presses
    }
    fn brute_forced_min(&self) -> Option<(u64, u64)> {
        let mut min_presses = None;
        for a in 0..=100 {
            for b in 0..=100 {
                let x = self.button_a.x_plus * a + self.button_b.x_plus * b;
                let y = self.button_a.y_plus * a + self.button_b.y_plus * b;
                if x == self.prize.x && y == self.prize.y {
                    min_presses = Some((a, b));
                    break;
                }
            }
        }
        min_presses
    }
    fn token_cost(&self) -> Option<(u64, u64)> {
        self.brute_forced_min().map(|(a, b)| {
            let a_cost = a * 3;
            let b_cost = b * 1;
            (a_cost, b_cost)
        })
    }
}
impl TryFrom<&[&str]> for ClawGame {
    type Error = &'static str;
    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        let button_a = match ClawInstruction::try_from(value[0]) {
            Ok(ClawInstruction::ButtonA(button)) => button,
            _ => return Err("Invalid Instruction"),
        };
        let button_b = match ClawInstruction::try_from(value[1]) {
            Ok(ClawInstruction::ButtonB(button)) => button,
            _ => return Err("Invalid Instruction"),
        };
        let prize = match ClawInstruction::try_from(value[2]) {
            Ok(ClawInstruction::Prize(prize)) => prize,
            _ => return Err("Invalid Instruction"),
        };
        Ok(Self {
            button_a,
            button_b,
            prize,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ClawInstruction {
    ButtonA(Button),
    ButtonB(Button),
    Prize(Prize),
}
impl TryFrom<&str> for ClawInstruction {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (instruction, value) = value.split_once(":").ok_or("Invalid Instruction")?;
        match instruction.trim() {
            "Button A" => Ok(Self::ButtonA(value.try_into()?)),
            "Button B" => Ok(Self::ButtonB(value.try_into()?)),
            "Prize" => Ok(Self::Prize(value.try_into()?)),
            _ => Err("Invalid Instruction"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Button {
    x_plus: u64,
    y_plus: u64,
}
impl TryFrom<&str> for Button {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x_instruction, y_instruction) = value.split_once(",").ok_or("Invalid Button")?;
        let x_plus = x_instruction
            .trim()
            .strip_prefix("X+")
            .ok_or("Invalid Button")?;
        let y_plus = y_instruction
            .trim()
            .strip_prefix("Y+")
            .ok_or("Invalid Button")?;
        Ok(Self {
            x_plus: x_plus.parse().map_err(|_| "Invalid Button")?,
            y_plus: y_plus.parse().map_err(|_| "Invalid Button")?,
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Prize {
    x: u64,
    y: u64,
}
impl TryFrom<&str> for Prize {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x_instruction, y_instruction) = value.split_once(",").ok_or("Invalid Prize")?;
        let x = x_instruction
            .trim()
            .strip_prefix("X=")
            .ok_or("Invalid Prize")?;
        let y = y_instruction
            .trim()
            .strip_prefix("Y=")
            .ok_or("Invalid Prize")?;
        Ok(Self {
            x: x.parse().map_err(|_| "Invalid Prize")?,
            y: y.parse().map_err(|_| "Invalid Prize")?,
        })
    }
}
