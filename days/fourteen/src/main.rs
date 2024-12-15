use std::{collections::HashSet, fmt::Display, io::Write};

use colored::Colorize;

fn main() {
    println!("Hello, advent of code day fourteen!");
    let robots: Vec<GuardRobot> = include_str!("../input.txt")
        .lines()
        .enumerate()
        .map(|line| GuardRobot::try_from(line).unwrap())
        .collect();
    println!("{}", robots.len());
    let mut floor = BathroomFloor {
        height: 103,
        width: 101,
        robots,
        quadrants: vec![],
    };
    floor.move_robots(100);
    floor.get_quadrants();
    floor.fill_quadrants();
    let safety_factor = floor
        .quadrants
        .iter()
        .enumerate()
        .fold(1, |acc, (i, quadrant)| {
            println!("Quadrant: {}", i);
            println!("Robots: {}", quadrant.robots.len());
            println!();
            acc * quadrant.robots.len()
        });
    println!();
    println!("Safety Factor: {}", safety_factor);

    println!();
    println!();
    println!("Part Two");
    let new_robots: Vec<GuardRobot> = include_str!("../input.txt")
        .lines()
        .enumerate()
        .map(|line| GuardRobot::try_from(line).unwrap())
        .collect();
    let mut new_floor = BathroomFloor {
        height: 103,
        width: 101,
        robots: new_robots,
        quadrants: vec![],
    };
    let mut iterations = 1;
    let mut mins_safety_factor = std::i32::MAX;
    loop {
        new_floor.move_robots(iterations);
        new_floor.get_quadrants();
        new_floor.fill_quadrants();
        let safety_factor = new_floor
            .quadrants
            .iter()
            .enumerate()
            .fold(1, |acc, (i, quadrant)| acc * quadrant.robots.len() as i32);
        print!("Iterations: {} Safety: {}  {} \r " , iterations, safety_factor, new_floor);
        iterations += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const ROBOT_INSTRUCTIONS: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
    #[test]
    fn robot_paths() {
        println!();
        let robots: Vec<GuardRobot> = ROBOT_INSTRUCTIONS
            .lines()
            .enumerate()
            .map(|line| GuardRobot::try_from(line).unwrap())
            .collect();
        let mut floor = BathroomFloor {
            height: 7,
            width: 11,
            robots,
            quadrants: vec![],
        };
        floor.get_quadrants();
        println!("{}", floor);
        assert_eq!(floor.robots.len(), 12);
        floor.move_robots(100);
        println!("{}", floor);
        floor.fill_quadrants();
        let safety_factor = floor
            .quadrants
            .iter()
            .enumerate()
            .fold(1, |acc, (i, quadrant)| {
                println!("Quadrant: {}", i);
                println!("Robots: {}", quadrant.robots.len());
                println!();
                acc * quadrant.robots.len()
            });
        println!();
        println!("Safety Factor: {}", safety_factor);
        println!();
        assert_eq!(safety_factor, 12);
    }
}

#[derive(Debug)]
struct BathroomQuadrant {
    start: RobotPosition,
    end: RobotPosition,
    robots: Vec<GuardRobot>,
}
struct BathroomFloor {
    width: i32,
    height: i32,
    robots: Vec<GuardRobot>,
    quadrants: Vec<BathroomQuadrant>,
}
impl BathroomFloor {
    fn move_robots(&mut self, times: i32) {
        for robot in self.robots.iter_mut() {
            robot.move_x_times(times, (self.width, self.height));
        }
    }
    fn fill_quadrants(&mut self) {
        for robot in self.robots.iter() {
            for quadrant in self.quadrants.iter_mut() {
                let robot_x = robot.position.x;
                let robot_y = robot.position.y;
                if robot_x >= quadrant.start.x
                    && robot_x <= quadrant.end.x
                    && robot_y >= quadrant.start.y
                    && robot_y <= quadrant.end.y
                {
                    quadrant.robots.push(*robot);
                }
            }
        }
    }
    fn empty_quadrants(&mut self) {
        for quadrant in self.quadrants.iter_mut() {
            quadrant.robots.clear();
        }
    }
    fn get_quadrants(&mut self) {
        let mut quadrants: Vec<BathroomQuadrant> = vec![];
        let top_left = BathroomQuadrant {
            start: RobotPosition { x: 0, y: 0 },
            end: RobotPosition {
                x: self.width / 2 - 1,
                y: self.height / 2 - 1,
            },
            robots: Vec::new(),
        };
        let top_right = BathroomQuadrant {
            start: RobotPosition {
                x: self.width / 2 + 1,
                y: 0,
            },
            end: RobotPosition {
                x: self.width,
                y: self.height / 2 - 1,
            },
            robots: Vec::new(),
        };
        let bottom_left = BathroomQuadrant {
            start: RobotPosition {
                x: 0,
                y: self.height / 2 + 1,
            },
            end: RobotPosition {
                x: self.width / 2 - 1,
                y: self.height,
            },
            robots: Vec::new(),
        };
        let bottom_right = BathroomQuadrant {
            start: RobotPosition {
                x: self.width / 2 + 1,
                y: self.height / 2 + 1,
            },
            end: RobotPosition {
                x: self.width,
                y: self.height,
            },
            robots: Vec::new(),
        };
        quadrants.push(top_left);
        quadrants.push(top_right);
        quadrants.push(bottom_left);
        quadrants.push(bottom_right);
        self.quadrants = quadrants;
    }
}
impl Display for BathroomFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let robot = self
                    .robots
                    .iter()
                    .find(|robot| robot.position.x == x && robot.position.y == y);
                match robot {
                    Some(_) => write!(f, "{}", "O".bright_green())?,
                    None => write!(f, "{}", " ".black())?,
                }
            }
            writeln!(f)?;
        }
        print!("\r");
        std::io::stdout().flush().unwrap();
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardRobot {
    index: i32,
    position: RobotPosition,
    velocity: RobotVelocity,
}
impl GuardRobot {
    fn move_x_times(&mut self, times: i32, bounds: (i32, i32)) {
        for _ in 0..times {
            self.position.x += self.velocity.x;
            self.position.y += self.velocity.y;
        }
        self.position.x = self.position.x.rem_euclid(bounds.0);
        self.position.y = self.position.y.rem_euclid(bounds.1);
    }
}
impl TryFrom<(usize, &str)> for GuardRobot {
    type Error = &'static str;
    fn try_from(value: (usize, &str)) -> Result<Self, Self::Error> {
        let (position_str, velocity_str) = value
            .1
            .trim()
            .split_once(' ')
            .ok_or("Invalid robot format, expected position velocity")?;
        let position = match RobotInstruction::try_from(position_str)? {
            RobotInstruction::Position(position) => position,
            _ => return Err("Invalid position instruction"),
        };
        let velocity = match RobotInstruction::try_from(velocity_str)? {
            RobotInstruction::Velocity(velocity) => velocity,
            _ => return Err("Invalid velocity instruction"),
        };
        let index = value.0 as i32;
        Ok(Self {
            index,
            position,
            velocity,
        })
    }
}
impl Display for GuardRobot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Robot: {}", self.index)?;
        writeln!(f, "{}", self.position)?;
        writeln!(f, "{}", self.velocity)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RobotInstruction {
    Position(RobotPosition),
    Velocity(RobotVelocity),
}
impl TryFrom<&str> for RobotInstruction {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (key, value) = value
            .split_once('=')
            .ok_or("Invalid instruction format, expected key=value")?;
        match key {
            "p" => Ok(Self::Position(RobotPosition::try_from(value)?)),
            "v" => Ok(Self::Velocity(RobotVelocity::try_from(value)?)),
            _ => Err("Invalid instruction key"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RobotPosition {
    x: i32,
    y: i32,
}
impl TryFrom<&str> for RobotPosition {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x, y) = value
            .split_once(',')
            .ok_or("Invalid position format, expected x,y")?;
        Ok(Self {
            x: x.parse().map_err(|_| "Invalid x value")?,
            y: y.parse().map_err(|_| "Invalid y value")?,
        })
    }
}
impl Display for RobotPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "At     X: {}, Y: {}",
            self.x.to_string().blue(),
            self.y.to_string().blue()
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RobotVelocity {
    x: i32,
    y: i32,
}
impl TryFrom<&str> for RobotVelocity {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x, y) = value
            .split_once(',')
            .ok_or("Invalid velocity format, expected x,y")?;
        Ok(Self {
            x: x.parse().map_err(|_| "Invalid x value")?,
            y: y.parse().map_err(|_| "Invalid y value")?,
        })
    }
}
impl Display for RobotVelocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Moving X: {}, Y: {} u/s",
            self.x.to_string().green(),
            self.y.to_string().green()
        )
    }
}
