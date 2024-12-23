fn main() {
    println!("Hello, advent of code day seventeen!");
}
#[cfg(test)]
mod tests {
    use super::*;
    const DEBUG_PROGRAM_TEST: &str = r#"
    Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;
    const DEBUG_PROGRAM: &str = r#"
    Register A: 53437164
Register B: 0
Register C: 0

Program: 2,4,1,7,7,5,4,1,1,4,5,5,0,3,3,0
"#;
    #[test]
    fn part_two() {
        let mut input = DEBUG_PROGRAM.trim().lines();
        let registrar = Registrar { a: 0, b: 0, c: 0 };
        let program_input = input
            .filter(|line| line.starts_with("Program:"))
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap();
        let instructions = program_input
            .trim()
            .chars()
            .filter_map(|c| c.try_into().ok())
            .collect::<Vec<ThreeBitNumber>>();
        println!("{:?}", instructions);
        let mut debug_program = DebugProgram {
            instruction_pointer: 0,
            register: registrar,
            instructions,
            output: vec![],
        };
        let mut iter = 0;
        loop {
            loop {
                match debug_program.debug() {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            if debug_program.output == debug_program.instructions {
                break;
            } else {
                iter += 1;
                debug_program.output = vec![];
                debug_program.register.a = iter;
                debug_program.register.b = 0;
                debug_program.register.c = 0;
                debug_program.instruction_pointer = 0;
            }
        }
        // print output joined by commas
        println!(
            "{:?}",
            debug_program
                .output
                .iter()
                .map(|x| x.num.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
    #[test]
    fn test() {
        let mut input = DEBUG_PROGRAM.trim().lines();
        let registrar_lines = input.by_ref().take(3);
        let mut registrar = Registrar { a: 0, b: 0, c: 0 };
        registrar_lines.for_each(|line| {
            let mut line = line.trim().split_whitespace();
            let register = line.nth(1).unwrap().chars().nth(0).unwrap();
            let value = line.last().unwrap().parse::<u32>().unwrap();
            match register {
                'A' => registrar.a = value,
                'B' => registrar.b = value,
                'C' => registrar.c = value,
                _ => panic!("Invalid register"),
            }
        });
        println!("{:?}", registrar);
        let program_input = input
            .filter(|line| line.starts_with("Program:"))
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap();
        let instructions = program_input
            .trim()
            .chars()
            .filter_map(|c| c.try_into().ok())
            .collect::<Vec<ThreeBitNumber>>();
        println!("{:?}", instructions);
        let mut debug_program = DebugProgram {
            instruction_pointer: 0,
            register: registrar,
            instructions,
            output: vec![],
        };
        let mut iter = 0;
        loop {
            match debug_program.debug() {
                Ok(_) => {}
                Err(_) => break,
            }
        }
        // print output joined by commas
        println!(
            "{:?}",
            debug_program
                .output
                .iter()
                .map(|x| x.num.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}
#[derive(Debug)]
struct DebugProgram {
    instruction_pointer: usize,
    register: Registrar,
    instructions: Vec<ThreeBitNumber>,
    output: Vec<ThreeBitNumber>,
}
impl DebugProgram {
    fn debug(&mut self) -> Result<(), ()> {
        let opcode = self.instructions.get(self.instruction_pointer).ok_or(())?;
        let operand = self
            .instructions
            .get(self.instruction_pointer + 1)
            .ok_or(())?;
        let opcode = OpCode::from(*opcode);
        match opcode {
            OpCode::Adv => {
                let combo_operand = match ComboOperand::try_from(*operand).unwrap() {
                    ComboOperand::Value(value) => value,
                    ComboOperand::RegisterA => self.register.a,
                    ComboOperand::RegisterB => self.register.b,
                    ComboOperand::RegisterC => self.register.c,
                };
                self.register.a =
                    (self.register.a as f32 / 2u32.pow(combo_operand) as f32).trunc() as u32;
                self.instruction_pointer += 2;
            }
            OpCode::Bxl => {
                self.register.b = self.register.b ^ operand.num;
                self.instruction_pointer += 2;
            }
            OpCode::BSt => {
                let combo_operand = match ComboOperand::try_from(*operand).unwrap() {
                    ComboOperand::Value(value) => value,
                    ComboOperand::RegisterA => self.register.a,
                    ComboOperand::RegisterB => self.register.b,
                    ComboOperand::RegisterC => self.register.c,
                };
                self.register.b = combo_operand % 8;
                self.instruction_pointer += 2;
            }
            OpCode::Jnz => {
                if self.register.a != 0 {
                    self.instruction_pointer = operand.num as usize;
                } else {
                    self.instruction_pointer += 2;
                }
            }
            OpCode::Bxc => {
                self.register.b = self.register.b ^ self.register.c;
                self.instruction_pointer += 2;
            }
            OpCode::Out => {
                let combo_operand = match ComboOperand::try_from(*operand).unwrap() {
                    ComboOperand::Value(value) => value,
                    ComboOperand::RegisterA => self.register.a,
                    ComboOperand::RegisterB => self.register.b,
                    ComboOperand::RegisterC => self.register.c,
                };
                let value = combo_operand % 8;
                self.output.push(ThreeBitNumber { num: value });
                self.instruction_pointer += 2;
            }
            OpCode::Bdv => {
                let combo_operand = match ComboOperand::try_from(*operand).unwrap() {
                    ComboOperand::Value(value) => value,
                    ComboOperand::RegisterA => self.register.a,
                    ComboOperand::RegisterB => self.register.b,
                    ComboOperand::RegisterC => self.register.c,
                };
                self.register.b =
                    (self.register.a as f32 / 2u32.pow(combo_operand) as f32).trunc() as u32;
                self.instruction_pointer += 2;
            }
            OpCode::Cdv => {
                let combo_operand = match ComboOperand::try_from(*operand).unwrap() {
                    ComboOperand::Value(value) => value,
                    ComboOperand::RegisterA => self.register.a,
                    ComboOperand::RegisterB => self.register.b,
                    ComboOperand::RegisterC => self.register.c,
                };
                self.register.c =
                    (self.register.a as f32 / 2u32.pow(combo_operand) as f32).trunc() as u32;
                self.instruction_pointer += 2;
            }
        }
        Ok(())
    }
}
#[derive(Debug)]
struct Registrar {
    a: u32,
    b: u32,
    c: u32,
}
enum ComboOperand {
    Value(u32),
    RegisterA,
    RegisterB,
    RegisterC,
}
impl TryFrom<ThreeBitNumber> for ComboOperand {
    type Error = &'static str;
    fn try_from(value: ThreeBitNumber) -> Result<Self, Self::Error> {
        match value.num {
            0 => Ok(ComboOperand::Value(0)),
            1 => Ok(ComboOperand::Value(1)),
            2 => Ok(ComboOperand::Value(2)),
            3 => Ok(ComboOperand::Value(3)),
            4 => Ok(ComboOperand::RegisterA),
            5 => Ok(ComboOperand::RegisterB),
            6 => Ok(ComboOperand::RegisterC),
            _ => Err("Invalid operand"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ThreeBitNumber {
    num: u32,
}
#[derive(Debug, Clone, Copy)]
enum OpCode {
    Adv,
    Bxl,
    BSt,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
impl From<ThreeBitNumber> for OpCode {
    fn from(value: ThreeBitNumber) -> Self {
        match value.num {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::BSt,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => panic!("Invalid opcode"),
        }
    }
}
impl TryFrom<char> for ThreeBitNumber {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(ThreeBitNumber { num: 0 }),
            '1' => Ok(ThreeBitNumber { num: 1 }),
            '2' => Ok(ThreeBitNumber { num: 2 }),
            '3' => Ok(ThreeBitNumber { num: 3 }),
            '4' => Ok(ThreeBitNumber { num: 4 }),
            '5' => Ok(ThreeBitNumber { num: 5 }),
            '6' => Ok(ThreeBitNumber { num: 6 }),
            '7' => Ok(ThreeBitNumber { num: 7 }),
            _ => Err("Invalid digit"),
        }
    }
}
