use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::DivAssign,
};

use colored::Colorize;

fn main() {
    println!("Hello, advent of code day fifteen!");
    let mut lines = include_str!("../input.txt").trim().lines();

    let mut warehouse = Warehouse::try_from(
        lines
            .by_ref()
            .take_while(|l| !l.trim().is_empty())
            .collect::<Vec<&str>>()
            .join("\n")
            .as_str(),
    )
    .unwrap();
    let instruction_set =
        RobotInstructionSet::try_from(lines.collect::<Vec<&str>>().join("\n").as_str()).unwrap();
    println!();
    println!("{}", warehouse);
    println!();
    println!("{}", instruction_set);
    println!();
    println!();
    instruction_set
        .instructions
        .iter()
        .enumerate()
        .for_each(|(count, instruction)| {
            warehouse.move_robot(*instruction);
             if count > 0 {
                 std::thread::sleep(std::time::Duration::from_millis(30));
                 for _ in 0..warehouse.warehouse_height() {
                     print!("\x1b[1A"); // Move cursor up by 1 line
                     print!("\x1b[2K"); // Clear the line
                 }
             }
             print!("{}", warehouse);
        });
    println!();
    println!("GPS Sum: {}", warehouse.gps_sum());
}
#[cfg(test)]
mod tests {

    const WAREHOUSE_MAP: &str = r#"
    ##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########
"#;
    #[test]
    fn warehouse_map() {
        let warehouse = super::Warehouse::try_from(WAREHOUSE_MAP).unwrap();
        println!();
        println!("{}", warehouse);
        assert_eq!(warehouse.warehouse_width(), 10);
        assert_eq!(warehouse.warehouse_height(), 10);
    }
    const ROBOT_INSTRUCTION_SET: &str = r#"
    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;
    #[test]
    fn robot_instruction_set() {
        let instruction_set = super::RobotInstructionSet::try_from(ROBOT_INSTRUCTION_SET).unwrap();
        println!();
        println!("{}", instruction_set);
        assert_eq!(instruction_set.instructions.len(), 700);
    }
    const TEST_INPUT: &str = r#"
    ########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"#;
    const TEST_RESULT: &str = r#"
    ########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
"#;
    #[test]
    fn move_warehouse_robot_small() {
        // split lines at the empty one
        //
        let mut lines = TEST_INPUT.trim().lines();
        let mut warehouse = super::Warehouse::try_from(
            lines
                .by_ref()
                .take_while(|l| !l.trim().is_empty())
                .collect::<Vec<&str>>()
                .join("\n")
                .as_str(),
        )
        .unwrap();
        let instruction_set =
            super::RobotInstructionSet::try_from(lines.collect::<Vec<&str>>().join("\n").as_str())
                .unwrap();
        println!();
        println!("{}", warehouse);
        println!();
        println!("{}", instruction_set);
        println!();
        println!();
        instruction_set
            .instructions
            .iter()
            .enumerate()
            .for_each(|(count, instruction)| {
                warehouse.move_robot(*instruction);
                if count > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(30));
                    for _ in 0..warehouse.warehouse_height() {
                        print!("\x1b[1A"); // Move cursor up by 1 line
                        print!("\x1b[2K"); // Clear the line
                    }
                }
                print!("{}", warehouse);
            });
        let test_result = super::Warehouse::try_from(TEST_RESULT).unwrap();
        println!();
        println!("EXPECTED Result");
        println!("{}", test_result);
        assert_eq!(warehouse.map, test_result.map);
        assert_eq!(warehouse.gps_sum(), 2028);
    }
    const LARGE_TEST_RESULT: &str = r#"
    ##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
"#;
    #[test]
    fn move_warehouse_robot_large() {
        let mut warehouse = super::Warehouse::try_from(WAREHOUSE_MAP).unwrap();
        let instruction_set = super::RobotInstructionSet::try_from(ROBOT_INSTRUCTION_SET).unwrap();
        println!();
        println!("{}", warehouse);
        println!();
        println!("{}", instruction_set);
        println!();
        println!();
        instruction_set
            .instructions
            .iter()
            .enumerate()
            .for_each(|(count, instruction)| {
                warehouse.move_robot(*instruction);
                if count > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(20));
                    for _ in 0..warehouse.warehouse_height() {
                        print!("\x1b[1A"); // Move cursor up by 1 line
                        print!("\x1b[2K"); // Clear the line
                    }
                }
                print!("{}", warehouse);
            });
        let test_result = super::Warehouse::try_from(LARGE_TEST_RESULT).unwrap();
        println!();
        println!("EXPECTED Result");
        println!("{}", test_result);
        assert_eq!(warehouse.map, test_result.map);
        assert_eq!(warehouse.gps_sum(), 10092);
    }
}
struct RobotInstructionSet {
    instructions: Vec<RobotInstruction>,
}
struct Warehouse {
    map: HashMap<WarehousePosition, WarehouseTile>,
}
impl Warehouse {
    fn warehouse_width(&self) -> usize {
        self.map.keys().map(|p| p.x).max().unwrap() as usize + 1
    }
    fn warehouse_height(&self) -> usize {
        self.map.keys().map(|p| p.y).max().unwrap() as usize + 1
    }
    fn gps_sum(&self) -> u64 {
        self.map
            .iter()
            .filter(|(_, tile)| **tile == WarehouseTile::GoodsBox)
            .map(|(position, _)| position.gps_value())
            .sum()
    }
    fn move_robot(&mut self, instruction: RobotInstruction) {
        let robot_position = self
            .map
            .iter()
            .find(|(_, tile)| **tile == WarehouseTile::Robot)
            .unwrap()
            .0
            .clone();
        let new_robot_position = robot_position.move_position(instruction);
        if let Some(tile) = self.map.get(&new_robot_position) {
            match tile {
                WarehouseTile::Empty => {
                    self.map.insert(new_robot_position, WarehouseTile::Robot);
                    self.map.insert(robot_position, WarehouseTile::Empty);
                }
                WarehouseTile::GoodsBox => {
                    // Start checking the space after the box
                    let mut current_position = new_robot_position;
                    loop {
                        // Calculate the next position
                        let next_position = current_position.move_position(instruction);

                        // Check the tile at the next position
                        match self.map.get(&next_position) {
                            Some(WarehouseTile::Empty) => {
                                // We found an empty space, move the box and the robot
                                self.map.insert(next_position, WarehouseTile::GoodsBox);

                                // Move the robot to the new position
                                self.map.insert(new_robot_position, WarehouseTile::Robot);
                                self.map.insert(robot_position, WarehouseTile::Empty);
                                break; // Exit the loop as we've moved the box
                            }
                            Some(WarehouseTile::GoodsBox) => {
                                // If the next space is another box, continue checking the next position
                                current_position = next_position;
                            }
                            Some(WarehouseTile::Wall) => {
                                break;
                            }
                            _ => break,
                        }
                    }
                }
                _ => {}
            }
        };
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WarehouseTile {
    GoodsBox,
    Empty,
    Wall,
    Robot,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RobotInstruction {
    MoveUp,
    MoveLeft,
    MoveRight,
    MoveDown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct WarehousePosition {
    x: u64,
    y: u64,
}
impl WarehousePosition {
    fn gps_value(&self) -> u64 {
        (self.y * 100) + self.x
    }
    fn move_position(&self, instruction: RobotInstruction) -> Self {
        match instruction {
            RobotInstruction::MoveUp => WarehousePosition {
                x: self.x,
                y: self.y - 1,
            },
            RobotInstruction::MoveDown => WarehousePosition {
                x: self.x,
                y: self.y + 1,
            },
            RobotInstruction::MoveLeft => WarehousePosition {
                x: self.x - 1,
                y: self.y,
            },
            RobotInstruction::MoveRight => WarehousePosition {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

/////////////////////
// PARSING METHODS //
// //////////////////
impl TryFrom<&str> for Warehouse {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut map = HashMap::new();
        value.trim().lines().enumerate().for_each(|(y, line)| {
            line.trim().chars().enumerate().for_each(|(x, c)| {
                let position = WarehousePosition {
                    x: x.to_string().parse().unwrap(),
                    y: y.to_string().parse().unwrap(),
                };
                let tile = WarehouseTile::try_from(c).unwrap();
                map.insert(position, tile);
            });
        });
        Ok(Warehouse { map })
    }
}
impl TryFrom<&str> for RobotInstructionSet {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let instructions = value
            .trim()
            .chars()
            .filter_map(|c| RobotInstruction::try_from(c).ok())
            .collect::<Vec<RobotInstruction>>();
        Ok(RobotInstructionSet { instructions })
    }
}
impl TryFrom<char> for WarehouseTile {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(WarehouseTile::Wall),
            '.' => Ok(WarehouseTile::Empty),
            'O' => Ok(WarehouseTile::GoodsBox),
            '@' => Ok(WarehouseTile::Robot),
            _ => Err("Invalid character"),
        }
    }
}
impl TryFrom<char> for RobotInstruction {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(RobotInstruction::MoveUp),
            '<' => Ok(RobotInstruction::MoveLeft),
            '>' => Ok(RobotInstruction::MoveRight),
            'v' => Ok(RobotInstruction::MoveDown),
            _ => Err("Invalid character"),
        }
    }
}
impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.warehouse_width();
        let height = self.warehouse_height();
        for y in 0..height {
            for x in 0..width {
                let tile = self
                    .map
                    .get(&WarehousePosition {
                        x: x as u64,
                        y: y as u64,
                    })
                    .unwrap();
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Display for WarehouseTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            WarehouseTile::Wall => "#".red(),
            WarehouseTile::Empty => ".".white(),
            WarehouseTile::GoodsBox => "O".yellow(),
            WarehouseTile::Robot => "@".bright_blue(),
        };
        write!(f, "{}", c)
    }
}
impl Display for RobotInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            RobotInstruction::MoveUp => "^".bright_green(),
            RobotInstruction::MoveLeft => "<".bright_blue(),
            RobotInstruction::MoveRight => ">".bright_magenta(),
            RobotInstruction::MoveDown => "v".bright_cyan(),
        };
        write!(f, "{}", c)
    }
}

impl Display for RobotInstructionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instruction in &self.instructions {
            write!(f, "{}", instruction)?;
        }
        Ok(())
    }
}
