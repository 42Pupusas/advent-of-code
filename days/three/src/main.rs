use std::io::{BufRead, Read};

fn open(filename: &str) -> impl BufRead {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .open(filename)
        .unwrap();

    std::io::BufReader::new(f)
}

pub fn read_by_byte(filename: &str) -> impl Iterator<Item = u8> {
    let mut buffer = open(filename);
    std::iter::from_fn(move || {
        let mut buf = [0u8; 1];
        buffer.read_exact(&mut buf).ok()?;
        Some(buf[0])
    })
}

fn main() {
    let input_file = read_by_byte("days/three/input.txt")
        .map(|c| c as char)
        .collect::<Vec<char>>();

    let mut mult_strs = input_file.iter().peekable();
    let mut enabled = false;
    let mut mul_instructions = 0;
    while let Some(c) = mult_strs.next() {
        if c == &'m' {
            if let Some('u') = mult_strs.next() {
                if let Some('l') = mult_strs.next() {
                    if let Some('(') = mult_strs.next() {
                        let mut instruction = String::new();
                        while let Some(c) = mult_strs.next_if(|c| c != &&')') {
                            if c.is_numeric() || c == &',' {
                                instruction.push(c.clone());
                            } else {
                                break;
                            }
                        }
                        if let Some((x_str, y_str)) = instruction.split_once(',') {
                            let x = x_str.parse::<i32>();
                            let y = y_str.parse::<i32>();
                            if let (Ok(x), Ok(y)) = (x, y) {
                                let mul = x * y;
                                if enabled {
                                    mul_instructions += mul;
                                }
                            }
                        }
                    }
                }
            }
        }
        if c == &'d' {
            if let Some('o') = mult_strs.next() {
                match mult_strs.next() {
                    Some('(') => {
                        if let Some(')') = mult_strs.next() {
                            enabled = true;
                        }
                    }
                    Some('n') => {
                        if let Some('\'') = mult_strs.next() {
                            if let Some('t') = mult_strs.next() {
                                if let Some('(') = mult_strs.next() {
                                    if let Some(')') = mult_strs.next() {
                                        enabled = false;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    println!("NEW {:?}", mul_instructions);
}

#[derive(Debug)]
pub struct MulInstruction {
    pub x: i32,
    pub y: i32,
}
impl MulInstruction {
    pub fn parse_mul_instruction(instruction: &str) -> Result<Self, String> {
        let instruction =
            instruction.trim_matches(|c| c == 'm' || c == 'u' || c == 'l' || c == '(' || c == ')');
        let (x, y) = instruction.split_once(',').ok_or("Invalid instruction")?;
        let x = x.parse::<i32>().map_err(|_| "Invalid x")?;
        let y = y.parse::<i32>().map_err(|_| "Invalid y")?;
        if x > 999 || y > 999 {
            return Err("Invalid x or y".to_string());
        }
        Ok(Self { x, y })
    }
    pub fn parse_instruction_set(instruction_set: &str) -> Vec<Self> {
        let mult_strs = instruction_set.split("mul").collect::<Vec<&str>>();
        mult_strs
            .iter()
            .filter_map(|s| {
                let stripped_instructions = s.split_inclusive(')').collect::<Vec<&str>>();
                let instruction = stripped_instructions.first()?;
                Self::parse_mul_instruction(instruction).ok()
            })
            .collect::<Vec<_>>()
    }
    pub fn multiply_instruction_set(instruction_set: &str) -> i32 {
        let instructions = Self::parse_instruction_set(instruction_set);
        println!("Instructions {:?}", instructions.len());
        instructions.iter().fold(0, |acc, x| acc + x.multiply())
    }
    pub fn multiply(&self) -> i32 {
        self.x * self.y
    }
}
#[cfg(test)]
mod tests {
    use crate::MulInstruction;

    #[test]
    fn parse_conditional_instruction_set() {}
    #[test]
    fn parse_mul_string() {
        pub const BROKEN_TEST_STR_1: &str = "mul(4*";
        pub const BROKEN_TEST_STR_2: &str = "mul(6,9!";
        pub const BROKEN_TEST_STR_3: &str = "?(12,34)";
        pub const BROKEN_TEST_STR_4: &str = "mul ( 2 , 4 )";
        pub const TEST_STR: &str = "mul(223,3)";
        pub const TEST_STR_1: &str = "(223,3)";
        pub const TEST_STR_2: &str = "mul(223,3))";
        let result = super::MulInstruction::parse_mul_instruction(TEST_STR);
        println!("{:?}", result);
        let result_2 = super::MulInstruction::parse_mul_instruction(TEST_STR_1);
        println!("{:?}", result);
        let result_3 = super::MulInstruction::parse_mul_instruction(TEST_STR_2);
        println!("{:?}", result);
        let err = super::MulInstruction::parse_mul_instruction(BROKEN_TEST_STR_1);
        println!("{:?}", err);
        let err2 = super::MulInstruction::parse_mul_instruction(BROKEN_TEST_STR_2);
        println!("{:?}", err2);
        let err3 = super::MulInstruction::parse_mul_instruction(BROKEN_TEST_STR_3);
        println!("{:?}", err3);
        let err4 = super::MulInstruction::parse_mul_instruction(BROKEN_TEST_STR_4);
        println!("{:?}", err4);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result_2.is_ok(), true);
        assert_eq!(result_3.is_ok(), true);
        assert_eq!(err.is_err(), true);
        assert_eq!(err2.is_err(), true);
        assert_eq!(err3.is_err(), true);
        assert_eq!(err4.is_err(), true);
    }
    #[test]
    fn parse_instruction_set() {
        pub const TEST_STR: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        // must split at 'mul' 'do' or 'dont'
        let mut mult_strs = TEST_STR.chars().peekable();
        println!("{:?}", mult_strs);
        let mut enabled = true;
        let mut mul_instructions = vec![];
        while let Some(c) = mult_strs.next() {
            if c == 'm' {
                if let Some('u') = mult_strs.next() {
                    if let Some('l') = mult_strs.next() {
                        if let Some('(') = mult_strs.next() {
                            let mut instruction = String::new();
                            while let Some(c) = mult_strs.next_if(|c| c != &')') {
                                if c.is_numeric() || c == ',' {
                                    instruction.push(c);
                                } else {
                                    break;
                                }
                            }
                            println!("found mul {}", instruction);
                            if let Some((x_str, y_str)) = instruction.split_once(',') {
                                let x = x_str.parse::<i32>();
                                let y = y_str.parse::<i32>();
                                if let (Ok(x), Ok(y)) = (x, y) {
                                    println!("x is {} y is {}", x, y);
                                    let mul = MulInstruction { x, y };
                                    if enabled {
                                        mul_instructions.push(mul);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if c == 'd' {
                if let Some('o') = mult_strs.next() {
                    match mult_strs.next() {
                        Some('(') => {
                            if let Some(')') = mult_strs.next() {
                                println!("found do");
                                enabled = true;
                            }
                        }
                        Some('n') => {
                            if let Some('\'') = mult_strs.next() {
                                if let Some('t') = mult_strs.next() {
                                    if let Some('(') = mult_strs.next() {
                                        if let Some(')') = mult_strs.next() {
                                            println!("found dont");
                                            enabled = false;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        let total = mul_instructions.iter().fold(0, |acc, x| acc + x.multiply());
        println!("{:?}", total);
    }
    #[test]
    fn day_three_test() {
        pub const TEST_STR: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let total = super::MulInstruction::multiply_instruction_set(TEST_STR);
        println!("{:?}", total);
        assert_eq!(total, 161);
    }
}
