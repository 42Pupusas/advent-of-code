use std::{collections::HashMap, fmt::Display};

fn main() {
    println!("Hello, advent of code day 21!");
    let input_str = include_str!("../input.txt");
    let codes = input_str.lines().collect::<Vec<&str>>();
    let lock_keypad = LockKeypad::default();
    let robot_keypad = RobotKeypad::default();
    let mut total_complexity = 0;
    for code in codes {
        let pushes = lock_keypad.find_button_pushes_for_code(code);
        let code_numeric = code
            .chars()
            .filter_map(|c| c.is_numeric().then(|| c))
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let first_robot_pushes = robot_keypad.find_button_pushes_for_robot_instructions(pushes);
        let second_robot_pushes =
            robot_keypad.find_button_pushes_for_robot_instructions(first_robot_pushes);
        let code_complexity = code_numeric * second_robot_pushes.len() as i32;
        total_complexity += code_complexity;
    }
    println!("Total complexity: {}", total_complexity);
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "980A";
    const TEST_RESULT: &str = "<A^A>^^AvvvA";
    const SECOND_RESULT: &str = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
    const THIRD_RESULT: &str =
        "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A";
    const FINAL_TEST_INPUT: &str = r#"
    029A
980A
179A
456A
379A
    "#;
    #[test]
    fn test_final_input() {
        let codes = FINAL_TEST_INPUT.trim().lines().collect::<Vec<&str>>();
        let lock_keypad = LockKeypad::default();
        let robot_keypad = RobotKeypad::default();
        let mut total_complexity = 0;
        for code in codes {
            let pushes = lock_keypad.find_button_pushes_for_code(code.trim());
            pushes.iter().for_each(|push| print!("{}", push));
            print!("\n");
            let code_numeric = code
                .chars()
                .filter_map(|c| c.is_numeric().then(|| c))
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let first_robot_pushes = robot_keypad.find_button_pushes_for_robot_instructions(pushes);
            first_robot_pushes.iter().for_each(|push| print!("{}", push));
            print!("\n");
            let second_robot_pushes =
                robot_keypad.find_button_pushes_for_robot_instructions(first_robot_pushes);
            second_robot_pushes.iter().for_each(|push| print!("{}", push));
            print!("\n");
            let code_complexity = code_numeric * second_robot_pushes.len() as i32;
            total_complexity += code_complexity;
        }
        assert_eq!(total_complexity, 126384);
    }
    #[test]
    fn print_keypads() {
        let lock_keypad = LockKeypad::default();
        println!("{}", lock_keypad);
        let robot_keypad = RobotKeypad::default();
        println!("{}", robot_keypad);
        let pushes = lock_keypad.find_button_pushes_for_code(TEST_INPUT);
        let code_numeric = TEST_INPUT
            .chars()
            .filter_map(|c| c.is_numeric().then(|| c))
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let second_robot_movement = robot_keypad.find_button_pushes_for_robot_instructions(pushes);
        second_robot_movement
            .iter()
            .for_each(|push| print!("{}", push));
        println!();
        let third_robot_movement =
            robot_keypad.find_button_pushes_for_robot_instructions(second_robot_movement);
        third_robot_movement
            .iter()
            .for_each(|push| print!("{}", push));
        print!("\n");
        let third_result_buttons: Vec<RobotKey> = THIRD_RESULT
            .chars()
            .filter_map(|c| c.try_into().ok())
            .collect::<Vec<RobotKey>>();
        third_result_buttons
            .iter()
            .for_each(|push| print!("{}", push));
        println!();
        assert_eq!(third_robot_movement.len(), third_result_buttons.len());
    }
}
enum Direction {
    Horizontal,
    Vertical,
}
struct RobotKeypad {
    keys: HashMap<KeypadPosition, RobotKey>,
}
impl RobotKeypad {
    fn is_gap(&self, pos: &KeypadPosition) -> bool {
        self.keys.get(pos).unwrap() == &RobotKey::Gap
    }
    fn find_button_pushes_for_robot_instructions(
        &self,
        instructions: Vec<RobotKey>,
    ) -> Vec<RobotKey> {
        let mut current_position = KeypadPosition { x: 2, y: 0 };
        let mut robot_instructions = Vec::new();
        let mut current_direction: Option<Direction> = None;

        for &next_target in &instructions {
            // Find the position of the next target key
            let target_position = self
                .keys
                .iter()
                .find(|(_, key)| key == &&next_target)
                .unwrap()
                .0;

            // Move towards the target position with the fewest changes in direction
            while &current_position != target_position {
                let move_left = current_position.x > target_position.x;
                let move_right = current_position.x < target_position.x;
                let move_up = current_position.y > target_position.y;
                let move_down = current_position.y < target_position.y;

                match current_direction {
                    // If the current direction is horizontal, prioritize horizontal movement
                    Some(Direction::Horizontal) => {
                        if move_left
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x - 1,
                                y: current_position.y,
                            })
                        {
                            robot_instructions.push(RobotKey::Left);
                            current_position.x -= 1;
                        } else if move_right
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x + 1,
                                y: current_position.y,
                            })
                        {
                            robot_instructions.push(RobotKey::Right);
                            current_position.x += 1;
                        } else if move_up
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x,
                                y: current_position.y - 1,
                            })
                        {
                            robot_instructions.push(RobotKey::Up);
                            current_position.y -= 1;
                            // Change direction to vertical after moving up
                            current_direction = Some(Direction::Vertical);
                        } else if move_down
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x,
                                y: current_position.y + 1,
                            })
                        {
                            robot_instructions.push(RobotKey::Down);
                            current_position.y += 1;
                            // Change direction to vertical after moving down
                            current_direction = Some(Direction::Vertical);
                        }
                    }
                    // If the current direction is vertical, prioritize vertical movement
                    Some(Direction::Vertical) => {
                        if move_up
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x,
                                y: current_position.y - 1,
                            })
                        {
                            robot_instructions.push(RobotKey::Up);
                            current_position.y -= 1;
                        } else if move_down
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x,
                                y: current_position.y + 1,
                            })
                        {
                            robot_instructions.push(RobotKey::Down);
                            current_position.y += 1;
                        } else if move_left
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x - 1,
                                y: current_position.y,
                            })
                        {
                            robot_instructions.push(RobotKey::Left);
                            current_position.x -= 1;
                            // Change direction to horizontal after moving left
                            current_direction = Some(Direction::Horizontal);
                        } else if move_right
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x + 1,
                                y: current_position.y,
                            })
                        {
                            robot_instructions.push(RobotKey::Right);
                            current_position.x += 1;
                            // Change direction to horizontal after moving right
                            current_direction = Some(Direction::Horizontal);
                        }
                    }
                    // If no direction is set, prioritize the axis of greater distance first
                    None => {
                        if (target_position.x - current_position.x).abs()
                            <= (target_position.y - current_position.y).abs()
                        {
                            current_direction = Some(Direction::Horizontal);
                        } else {
                            current_direction = Some(Direction::Vertical);
                        }
                    }
                }
            }

            // Once we've reached the target position, we press the key 'A'
            robot_instructions.push(RobotKey::A);
        }
        robot_instructions
    }
}
struct LockKeypad {
    keys: HashMap<KeypadPosition, LockKey>,
}
impl LockKeypad {
    fn is_gap(&self, pos: &KeypadPosition) -> bool {
        self.keys.get(pos).unwrap() == &LockKey::Gap
    }
    fn find_button_pushes_for_code(&self, code: &str) -> Vec<RobotKey> {
        let mut expected_pushes = code.chars().map(|c| match c {
            '0' => KeypadPosition { x: 1, y: 3 },
            '1' => KeypadPosition { x: 0, y: 2 },
            '2' => KeypadPosition { x: 1, y: 2 },
            '3' => KeypadPosition { x: 2, y: 2 },
            '4' => KeypadPosition { x: 0, y: 1 },
            '5' => KeypadPosition { x: 1, y: 1 },
            '6' => KeypadPosition { x: 2, y: 1 },
            '7' => KeypadPosition { x: 0, y: 0 },
            '8' => KeypadPosition { x: 1, y: 0 },
            '9' => KeypadPosition { x: 2, y: 0 },
            'A' => KeypadPosition { x: 2, y: 3 },
            _ => panic!("Invalid code character"),
        });
        let mut current_position = KeypadPosition { x: 2, y: 3 };
        let mut button_pushes = Vec::new();
        let mut current_direction: Option<Direction> = Some(Direction::Horizontal);
        while let Some(next_target) = expected_pushes.next() {
            while &current_position != &next_target {
                let move_left = current_position.x > next_target.x;
                let move_right = current_position.x < next_target.x;
                let move_up = current_position.y > next_target.y;
                let move_down = current_position.y < next_target.y;

                match current_direction {
                    Some(Direction::Horizontal) => {
                        if move_left
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x - 1,
                                y: current_position.y,
                            })
                        {
                            button_pushes.push(RobotKey::Left);
                            current_position.x -= 1;
                        } else if move_right
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x + 1,
                                y: current_position.y,
                            })
                        {
                            button_pushes.push(RobotKey::Right);
                            current_position.x += 1;
                        } else if move_up
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x,
                                y: current_position.y - 1,
                            })
                        {
                            button_pushes.push(RobotKey::Up);
                            current_position.y -= 1;
                            current_direction = Some(Direction::Vertical);
                        } else if move_down
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x,
                                y: current_position.y + 1,
                            })
                        {
                            button_pushes.push(RobotKey::Down);
                            current_position.y += 1;
                            current_direction = Some(Direction::Vertical);
                        }
                    }
                    Some(Direction::Vertical) => {
                        if move_up
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x,
                                y: current_position.y - 1,
                            })
                        {
                            button_pushes.push(RobotKey::Up);
                            current_position.y -= 1;
                        } else if move_down
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x,
                                y: current_position.y + 1,
                            })
                        {
                            button_pushes.push(RobotKey::Down);
                            current_position.y += 1;
                        } else if move_left
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x - 1,
                                y: current_position.y,
                            })
                        {
                            button_pushes.push(RobotKey::Left);
                            current_position.x -= 1;
                            current_direction = Some(Direction::Horizontal);
                        } else if move_right
                            && !self.is_gap(&KeypadPosition {
                                x: current_position.x + 1,
                                y: current_position.y,
                            })
                        {
                            button_pushes.push(RobotKey::Right);
                            current_position.x += 1;
                            current_direction = Some(Direction::Horizontal);
                        }
                    }
                    None => {
                        if (next_target.x - current_position.x).abs()
                            <= (next_target.y - current_position.y).abs()
                        {
                            current_direction = Some(Direction::Horizontal);
                        } else {
                            current_direction = Some(Direction::Vertical);
                        }
                    }
                }
            }
            button_pushes.push(RobotKey::A);
        }
        button_pushes
    }
}
impl Default for LockKeypad {
    fn default() -> Self {
        let mut keys = HashMap::new();
        keys.insert(KeypadPosition { x: 0, y: 0 }, LockKey::Seven);
        keys.insert(KeypadPosition { x: 1, y: 0 }, LockKey::Eight);
        keys.insert(KeypadPosition { x: 2, y: 0 }, LockKey::Nine);
        keys.insert(KeypadPosition { x: 0, y: 1 }, LockKey::Four);
        keys.insert(KeypadPosition { x: 1, y: 1 }, LockKey::Five);
        keys.insert(KeypadPosition { x: 2, y: 1 }, LockKey::Six);
        keys.insert(KeypadPosition { x: 0, y: 2 }, LockKey::One);
        keys.insert(KeypadPosition { x: 1, y: 2 }, LockKey::Two);
        keys.insert(KeypadPosition { x: 2, y: 2 }, LockKey::Three);
        keys.insert(KeypadPosition { x: 0, y: 3 }, LockKey::Gap);
        keys.insert(KeypadPosition { x: 1, y: 3 }, LockKey::Zero);
        keys.insert(KeypadPosition { x: 2, y: 3 }, LockKey::A);
        LockKeypad { keys }
    }
}
impl Display for LockKeypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..4 {
            for x in 0..3 {
                let key = self.keys.get(&KeypadPosition { x, y }).unwrap();
                write!(f, "* {} * ", key)?;
            }
            write!(f, "\n")?;
        }
        writeln!(f)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LockKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    Gap,
}
impl Display for LockKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = match self {
            LockKey::Zero => "0",
            LockKey::One => "1",
            LockKey::Two => "2",
            LockKey::Three => "3",
            LockKey::Four => "4",
            LockKey::Five => "5",
            LockKey::Six => "6",
            LockKey::Seven => "7",
            LockKey::Eight => "8",
            LockKey::Nine => "9",
            LockKey::A => "A",
            LockKey::Gap => " ",
        };
        write!(f, "{}", key)
    }
}
impl Default for RobotKeypad {
    fn default() -> Self {
        let mut keys = HashMap::new();
        keys.insert(KeypadPosition { x: 0, y: 0 }, RobotKey::Gap);
        keys.insert(KeypadPosition { x: 1, y: 0 }, RobotKey::Up);
        keys.insert(KeypadPosition { x: 2, y: 0 }, RobotKey::A);
        keys.insert(KeypadPosition { x: 0, y: 1 }, RobotKey::Left);
        keys.insert(KeypadPosition { x: 1, y: 1 }, RobotKey::Down);
        keys.insert(KeypadPosition { x: 2, y: 1 }, RobotKey::Right);
        RobotKeypad { keys }
    }
}
impl Display for RobotKeypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..2 {
            for x in 0..3 {
                let key = self.keys.get(&KeypadPosition { x, y }).unwrap();
                write!(f, "* {} * ", key)?;
            }
            write!(f, "\n")?;
        }
        writeln!(f)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RobotKey {
    Up,
    Down,
    Left,
    Right,
    A,
    Gap,
}
impl TryFrom<char> for RobotKey {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(RobotKey::Up),
            'v' => Ok(RobotKey::Down),
            '<' => Ok(RobotKey::Left),
            '>' => Ok(RobotKey::Right),
            'A' => Ok(RobotKey::A),
            ' ' => Ok(RobotKey::Gap),
            _ => Err("Invalid character"),
        }
    }
}
impl Display for RobotKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = match self {
            RobotKey::Up => "^",
            RobotKey::Down => "v",
            RobotKey::Left => "<",
            RobotKey::Right => ">",
            RobotKey::A => "A",
            RobotKey::Gap => " ",
        };
        write!(f, "{}", key)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct KeypadPosition {
    x: i32,
    y: i32,
}
