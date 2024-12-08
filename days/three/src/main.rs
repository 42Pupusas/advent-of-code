use colored::*;
use std::fmt::Display;

fn main() {
    println!("Hello, advent of code day three!");
    let input_file = include_str!("../input.txt");
    println!();
    println!();
    println!("Part one");
    println!();
    read_instructions(input_file, false);
    println!();
    println!();
    println!("Part two");
    println!();
    read_instructions(input_file, true);
    println!();
}
fn read_instructions(input: &str, control: bool) {
    let instructions = input
        .chars()
        .map(|c| OpChar::from(c))
        .collect::<Vec<OpChar>>();
    //println!("Instructions:");
    //instructions.iter().for_each(|x| {
    //    print!("{}", x);
    //});
    let mut index = 0;
    let mut op_instructions = InstructionSet {
        is_doing: true,
        instructions: Vec::new(),
    };

    while index < instructions.len() {
        let remaining = &instructions[index..]; // Slice remaining instructions

        // Try to parse the instructions
        if op_instructions.parse_instructions(remaining, control).is_ok() {
            // Find the closing parenthesis to advance the index properly
            index += remaining
                .iter()
                .position(|x| x == &OpChar::CloseParen)
                .unwrap_or(remaining.len())
                + 1; // Move past the closing parenthesis
        } else {
            // If no token is found, move to the next character
            index += 1;
        }
    }

    let total = op_instructions
        .instructions
        .iter()
        .map(|x| x.operate())
        .sum::<u64>();
    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INSTRUCTIONS: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))mul( 2 , 2 )";
    #[test]
    fn read_instruction_set() {
        let instructions = TEST_INSTRUCTIONS
            .chars()
            .map(|c| OpChar::from(c))
            .collect::<Vec<OpChar>>();
        instructions.iter().for_each(|x| {
            print!("{}", x);
        });
        println!();
        let mut index = 0;
        let mut op_instructions = InstructionSet {
            is_doing: true,
            instructions: Vec::new(),
        };
        while index < instructions.len() {
            let remaining = &instructions[index..]; // Slice remaining instructions
            if op_instructions.parse_instructions(remaining, false).is_ok() {
                index += remaining
                    .iter()
                    .position(|x| x == &OpChar::CloseParen)
                    .unwrap_or(remaining.len())
                    + 1;
            } else {
                // If no token is found, move to the next character
                index += 1;
            }
        }
        let total = op_instructions
            .instructions
            .iter()
            .map(|x| x.operate())
            .sum::<u64>();
        println!("Total: {}", total);
        assert_eq!(total, 161);
    }
    const PART_TWO_TEST: &str =
        r"#xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn read_instruction_set_part_two() {
        let instructions = PART_TWO_TEST
            .chars()
            .map(|c| OpChar::from(c))
            .collect::<Vec<OpChar>>();
        instructions.iter().for_each(|x| {
            print!("{}", x);
        });
        println!();
        let mut index = 0;
        let mut op_instructions = InstructionSet {
            is_doing: true,
            instructions: Vec::new(),
        };
        while index < instructions.len() {
            let remaining = &instructions[index..]; // Slice remaining instructions
            if op_instructions.parse_instructions(remaining, true).is_ok() {
                index += remaining
                    .iter()
                    .position(|x| x == &OpChar::CloseParen)
                    .unwrap_or(remaining.len())
                    + 1;
            } else {
                // If no token is found, move to the next character
                index += 1;
            }
        }
        let total = op_instructions
            .instructions
            .iter()
            .map(|x| x.operate())
            .sum::<u64>();
        println!("Total: {}", total);
        assert_eq!(total, 48);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum OpChar {
    M,
    U,
    L,
    Comma,
    Number(char),
    OpenParen,
    CloseParen,
    D,
    O,
    N,
    T,
    Apostrophe,
    NoOp,
}
impl From<&char> for OpChar {
    fn from(value: &char) -> Self {
        match value {
            'm' => Self::M,
            'u' => Self::U,
            'l' => Self::L,
            '(' => Self::OpenParen,
            ')' => Self::CloseParen,
            ',' => Self::Comma,
            'd' => Self::D,
            'o' => Self::O,
            'n' => Self::N,
            't' => Self::T,
            '\'' => Self::Apostrophe,
            _ => {
                if value.is_digit(10) {
                    Self::Number(*value)
                } else {
                    Self::NoOp
                }
            }
        }
    }
}
impl From<char> for OpChar {
    fn from(value: char) -> Self {
        match value {
            'm' => Self::M,
            'u' => Self::U,
            'l' => Self::L,
            '(' => Self::OpenParen,
            ')' => Self::CloseParen,
            ',' => Self::Comma,
            'd' => Self::D,
            'o' => Self::O,
            'n' => Self::N,
            't' => Self::T,
            '\'' => Self::Apostrophe,
            _ => {
                if value.is_digit(10) {
                    Self::Number(value)
                } else {
                    Self::NoOp
                }
            }
        }
    }
}
impl Display for OpChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::M => write!(f, "{}", "m".yellow()),
            Self::U => write!(f, "{}", "u".yellow()),
            Self::L => write!(f, "{}", "l".yellow()),
            Self::Comma => write!(f, "{}", ",".yellow()),
            Self::Number(c) => write!(f, "{}", c.to_string().blue()),
            Self::OpenParen => write!(f, "{}", "(".purple()),
            Self::CloseParen => write!(f, "{}", ")".purple()),
            Self::D => write!(f, "{}", "d".green()),
            Self::O => write!(f, "{}", "o".green()),
            Self::N => write!(f, "{}", "n".red()),
            Self::T => write!(f, "{}", "t".red()),
            Self::Apostrophe => write!(f, "{}", "'".red()),
            Self::NoOp => write!(f, " "),
        }
    }
}
enum OpToken {
    Mul,
    Do,
    DoNot,
}
impl TryFrom<&[&OpChar]> for OpToken {
    type Error = ();
    fn try_from(value: &[&OpChar]) -> Result<Self, Self::Error> {
        match value.len() {
            4 => match value.first() {
                Some(&OpChar::D) => {
                    if value[1] == &OpChar::O
                        && value[2] == &OpChar::OpenParen
                        && value[3] == &OpChar::CloseParen
                    {
                        Ok(Self::Do)
                    } else {
                        Err(())
                    }
                }
                Some(&OpChar::M) => {
                    if value[1] == &OpChar::U
                        && value[2] == &OpChar::L
                        && value[3] == &OpChar::OpenParen
                    {
                        Ok(Self::Mul)
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            },
            7 => {
                if value[0] == &OpChar::D
                    && value[1] == &OpChar::O
                    && value[2] == &OpChar::N
                    && value[3] == &OpChar::Apostrophe
                    && value[4] == &OpChar::T
                    && value[5] == &OpChar::OpenParen
                    && value[6] == &OpChar::CloseParen
                {
                    Ok(Self::DoNot)
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
enum OpInstruction {
    Mul(u64, u64),
}
impl OpInstruction {
    fn operate(&self) -> u64 {
        match self {
            Self::Mul(a, b) => {
                if a > &999 || b > &999 {
                    0
                } else {
                    a * b
                }
            }
        }
    }
}
impl Display for OpInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mul(a, b) => write!(f, "mul({}, {})", a, b),
        }
    }
}
pub struct InstructionSet {
    is_doing: bool,
    instructions: Vec<OpInstruction>,
}
impl InstructionSet {
    fn parse_instructions(&mut self, input: &[OpChar], controlablle: bool) -> Result<(), ()> {
        let mut chars = input.iter().peekable();
        while let Some(op_char) = chars.peek() {
            if op_char == &&OpChar::NoOp {
                chars.next();
                return Err(());
            }
            match op_char {
                &&OpChar::D => {
                    let mut word = Vec::new();
                    word.push(*op_char);
                    for _ in 0..7 {
                        chars.next(); // Skip 'u' and 'l'
                        match chars.peek() {
                            Some(OpChar::CloseParen) => {
                                word.push(&OpChar::CloseParen);
                                break;
                            }
                            Some(OpChar::NoOp) => return Err(()),
                            Some(next_op_char) => word.push(*next_op_char),
                            _ => return Err(()),
                        }
                    }
                    let word = OpToken::try_from(&word[..])?;
                    match word {
                        OpToken::Do => {
                            if controlablle && !self.is_doing {
                                self.is_doing = true;
                                return Ok(());
                            }
                        }
                        OpToken::DoNot => {
                            if controlablle && self.is_doing {
                                self.is_doing = false;
                                return Ok(());
                            }
                        }
                        _ => return Err(()),
                    }
                }
                &&OpChar::M => {
                    if !self.is_doing {
                        return Err(());
                    }
                    let mut word = Vec::new();
                    word.push(*op_char);
                    for _ in 0..3 {
                        chars.next(); // Skip 'u' and 'l'
                        match chars.peek() {
                            Some(OpChar::OpenParen) => {
                                word.push(&OpChar::OpenParen);
                                break;
                            }
                            Some(OpChar::NoOp) => return Err(()),
                            Some(next_op_char) => word.push(*next_op_char),
                            _ => return Err(()),
                        }
                    }
                    matches!(OpToken::try_from(&word[..])?, OpToken::Mul);
                    chars.next();
                    let mut value_one = String::new();
                    let mut value_two = String::new();
                    let mut found_comma = false;

                    while let Some(c) = chars.peek() {
                        match c {
                            OpChar::CloseParen => {
                                if found_comma {
                                    let new_instruction = OpInstruction::Mul(
                                        value_one.parse().unwrap(),
                                        value_two.parse().unwrap(),
                                    );
                                    self.instructions.push(new_instruction);
                                    return Ok(());
                                } else {
                                    return Err(()); // Missing comma between values
                                }
                            }
                            OpChar::Comma => {
                                if found_comma {
                                    return Err(()); // Multiple commas found
                                }
                                found_comma = true;
                                chars.next(); // consume the comma
                                continue;
                            }
                            OpChar::Number(c) => {
                                if found_comma {
                                    value_two.push(*c);
                                } else {
                                    value_one.push(*c);
                                }
                            }
                            _ => return Err(()), // Invalid character found
                        }
                        chars.next();
                    }
                }
                _ => {
                    chars.next();
                }
            }
            // If no valid "mul", just continue
            return Err(());
        }

        Err(()) // Return error if no valid "mul()" expression found
    }
}
