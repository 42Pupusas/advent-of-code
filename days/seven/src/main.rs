use std::{collections::HashSet, fmt::Display};

fn main() {
    println!("Hello, advent of code day seven!");
    println!();
    println!("Part one single thread");
    let time = std::time::Instant::now();
    part_one_single_thread();
    println!("Finished in: {:?}", time.elapsed());
    println!();
    println!("Part two single thread");
    let time = std::time::Instant::now();
    part_two_single_thread();
    println!("Finished in: {:?}", time.elapsed());
    println!();
    println!("Part one multi-threaded");
    let time = std::time::Instant::now();
    part_one_threaded();
    println!("Finished in: {:?}", time.elapsed());
    println!();
    println!("Part two multi-threaded");
    let time = std::time::Instant::now();
    part_two_threaded();
    println!("Finished in: {:?}", time.elapsed());
    println!();
}
fn part_one_single_thread() {
    let input = include_str!("../input.txt");
    let equations = input
        .lines()
        .filter_map(|x| BridgeEquation::try_from(x).ok())
        .collect::<Vec<_>>();
    let mut total = 0;
    println!("Checking equations");
    for eq in equations {
        if let Some(result) = eq.check_operation() {
            total += result;
        }
    }
    println!("Total: {}", total);
}
fn part_two_single_thread() {
    let input = include_str!("../input.txt");
    let equations = input
        .lines()
        .filter_map(|x| BridgeEquation::try_from(x).ok())
        .collect::<Vec<_>>();
    let mut total = 0;
    println!("Checking equations");
    for eq in equations {
        if let Some(result) = eq.check_complex_operation() {
            total += result;
        }
    }
    println!("Total: {}", total);
}
fn part_one_threaded() {
    let input = include_str!("../input.txt");
    let equations = input
        .lines()
        .filter_map(|x| BridgeEquation::try_from(x).ok())
        .collect::<Vec<_>>();
    let mut tasks = vec![];
    println!("Checking equations");
    for eq in equations {
        let task = std::thread::spawn(move || eq.check_operation());
        tasks.push(task);
    }
    println!("Waiting for results from {} tasks", tasks.len());
    let total: u64 = tasks
        .into_iter()
        .filter_map(|x| {
            let result = x.join().ok()??;
            Some(result)
        })
        .sum();
    println!("Total: {}", total);
}
fn part_two_threaded() {
    let input = include_str!("../input.txt");
    let equations = input
        .lines()
        .filter_map(|x| BridgeEquation::try_from(x).ok())
        .collect::<Vec<_>>();
    let mut tasks = vec![];
    println!("Checking equations");
    for eq in equations {
        let task = std::thread::spawn(move || eq.check_complex_operation());
        tasks.push(task);
    }
    println!("Waiting for results from {} tasks", tasks.len());
    let total: u64 = tasks
        .into_iter()
        .filter_map(|x| {
            let result = x.join().ok()??;
            Some(result)
        })
        .sum();
    println!("Total: {}", total);
}

#[derive(Debug)]
enum Operator {
    Add((u64, u64)),
    Multiply((u64, u64)),
    Cat((u64, u64)),
}
impl Operator {
    fn operate(&self) -> u64 {
        match self {
            Self::Add((a, b)) => a + b,
            Self::Multiply((a, b)) => a * b,
            Self::Cat((a, b)) => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct OperandList {
    operands: Vec<u64>,
}
impl OperandList {
    fn simple_operation(&self, target: u64) -> bool {
        let mut results = HashSet::new();
        self.operands.iter().for_each(|x| {
            if results.is_empty() {
                results.insert(*x);
                return;
            }
            let mut new_results = HashSet::new();
            results.iter().for_each(|y| {
                let add = Operator::Add((*x, *y)).operate();
                let mul = Operator::Multiply((*x, *y)).operate();
                new_results.insert(add);
                new_results.insert(mul);
            });
            results = new_results;
            if results.contains(&target) || results.iter().max().unwrap() > &target {
                return;
            }
        });
        results.contains(&target)
    }
    fn complex_operation(&self, target: u64) -> bool {
        let mut results = HashSet::new();
        self.operands.iter().for_each(|x| {
            if results.is_empty() {
                results.insert(*x);
                return;
            }
            let mut new_results = HashSet::new();
            results.iter().for_each(|y| {
                let add = Operator::Add((*x, *y)).operate();
                let mul = Operator::Multiply((*x, *y)).operate();
                let cat = Operator::Cat((*y, *x)).operate();
                new_results.insert(add);
                new_results.insert(mul);
                new_results.insert(cat);
            });
            results = new_results;
            if results.contains(&target) || results.iter().max().unwrap() > &target {
                return;
            }
        });
        results.contains(&target)
    }
}
impl TryFrom<&str> for OperandList {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let operands = value
            .split_whitespace()
            .filter_map(|x| x.parse::<u64>().ok())
            .collect();
        Ok(Self { operands })
    }
}

#[derive(Debug)]
struct BridgeEquation {
    result: u64,
    operands: OperandList,
}
impl Display for BridgeEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            format!("\x1b[32m{}\x1b[0m", self.result),
            self.operands
                .operands
                .iter()
                .map(|x| format!("\x1b[34m{}\x1b[0m", x)) // Blue for each operand
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
impl BridgeEquation {
    fn check_operation(self) -> Option<u64> {
        if self.operands.simple_operation(self.result) {
            Some(self.result as u64)
        } else {
            None
        }
    }
    fn check_complex_operation(self) -> Option<u64> {
        if self.operands.complex_operation(self.result) {
            Some(self.result as u64)
        } else {
            None
        }
    }
}
impl TryFrom<&str> for BridgeEquation {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (ruslt_str, oprands_str) = value.split_once(": ").ok_or("Invalid input")?;
        let result = ruslt_str.parse::<u64>().map_err(|_| "Invalid input")?;
        let operands = OperandList::try_from(oprands_str)?;
        Ok(Self { result, operands })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STR_PART_ONE: &str = r"#
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
        #";
    #[test]
    fn read_equations() {
        let equations = TEST_STR_PART_ONE
            .lines()
            .filter_map(|x| BridgeEquation::try_from(x).ok())
            .collect::<Vec<_>>();
        equations.iter().for_each(|x| println!("{:?}", x));
        assert_eq!(equations.len(), 9);
    }
    #[test]
    fn operate_equations() {
        let equations = TEST_STR_PART_ONE
            .lines()
            .filter_map(|x| BridgeEquation::try_from(x).ok())
            .collect::<Vec<_>>();
        equations.iter().for_each(|x| println!("{:?}", x));
        let mut total = 0;
        for eq in equations {
            if let Some(result) = eq.check_operation() {
                total += result;
            }
        }
        assert_eq!(total, 3749);
    }
    #[test]
    fn operate_complex_equations() {
        let equations = TEST_STR_PART_ONE
            .lines()
            .filter_map(|x| BridgeEquation::try_from(x).ok())
            .collect::<Vec<_>>();
        let mut total = 0;
        for eq in equations {
            if let Some(result) = eq.check_complex_operation() {
                total += result;
            }
        }
        assert_eq!(total, 11387);
    }
}
