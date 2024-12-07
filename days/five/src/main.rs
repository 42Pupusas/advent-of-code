fn main() {
    println!("Hello, world day 5, SARITAH IS A GENIUS!");
    let input_text = std::fs::read_to_string("days/five/input.txt").expect("Error reading file");
    let lines = input_text.split('\n').collect::<Vec<&str>>();
    let mut rule_list = Vec::new();
    let mut update_list = Vec::new();
    lines.iter().for_each(|line| {
        if line.contains('|') {
            if let Ok(rule_set) = SafetyRule::try_from(*line) {
                rule_list.push(rule_set);
            }
        } else {
            if let Ok(update) = SafetyUpdate::try_from(*line) {
                update_list.push(update);
            }
        }
    });
    println!("Rule list: {}", rule_list.len());
    println!();
    println!("Update list: {}", update_list.len());

    let mut total = 0;
    let mut unordered_total = 0;
    update_list.iter_mut().for_each(|update| {
        let update_graph = SafetyGraph::new(&rule_list, &update);
        if update.check_order(&update_graph) {
            total += update.middle();
        } else {
            update.sort_by_graph(&update_graph);
            unordered_total += update.middle();
        }
    });
    println!();
    println!("Total: {}", total);

    println!();
    println!("Unordered total: {}", unordered_total);
}
#[derive(Debug)]
struct SafetyRule {
    x: u32,
    y: u32,
}
impl SafetyRule {
    pub fn contains_x_and_y(&self, list: &SafetyUpdate) -> bool {
        list.list.contains(&self.x) && list.list.contains(&self.y)
    }
}
impl TryFrom<&str> for SafetyRule {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let error_str = format!("Invalid rule: {}", value);
        let trimmed_value = value.trim();
        let split_str = trimmed_value.split_once('|').ok_or(&error_str)?;
        let x = split_str
            .0
            .parse::<u32>()
            .map_err(|e| format!("Invalid x: {} at rule {}", e, value))?;
        let y = split_str
            .1
            .parse::<u32>()
            .map_err(|e| format!("Invalid y: {} at rule {}", e, value))?;
        Ok(SafetyRule { x, y })
    }
}

use std::collections::HashMap;
#[derive(Debug)]
struct SafetyGraph {
    in_degree: HashMap<u32, usize>, // keeps track of how many numbers a given number depends on
}
impl SafetyGraph {
    fn new(rule_set: &Vec<SafetyRule>, update: &SafetyUpdate) -> Self {
        let mut new_graph = SafetyGraph {
            in_degree: HashMap::new(),
        };
        rule_set.iter().for_each(|rule| {
            if rule.contains_x_and_y(update) {
                *new_graph.in_degree.entry(rule.y).or_insert(0) += 1;
                new_graph.in_degree.entry(rule.x).or_insert(0);
            }
        });
        new_graph
    }
}

#[derive(Debug)]
struct SafetyUpdate {
    list: Vec<u32>,
}
impl SafetyUpdate {
    pub fn middle(&self) -> u32 {
        let middle_point = self.list.len() / 2;
        self.list[middle_point]
    }
    pub fn check_order(&self, graph: &SafetyGraph) -> bool {
        let mut lastlevel_of_nodes = None;
        let mut in_order = true;
        self.list.iter().for_each(|node| {
            if let Some(node_level) = graph.in_degree.get(node) {
                match lastlevel_of_nodes {
                    Some(last_level) => {
                        if node_level > last_level {
                            lastlevel_of_nodes = Some(node_level);
                        } else {
                            in_order = false;
                        }
                    }
                    None => {
                        lastlevel_of_nodes = Some(node_level);
                    }
                }
            }
        });
        in_order
    }
    pub fn sort_by_graph(&mut self, graph: &SafetyGraph) {
        self.list.sort_by(|a, b| {
            let a_level = graph.in_degree.get(a).unwrap();
            let b_level = graph.in_degree.get(b).unwrap();
            a_level.cmp(b_level)
        });
    }
}
impl TryFrom<&str> for SafetyUpdate {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let trimmed_value = value.trim();
        let list = trimmed_value
            .split(',')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        if list.is_empty() {
            return Err("Empty list".to_string());
        }
        Ok(SafetyUpdate { list })
    }
}

#[cfg(test)]
mod tests {
    const RULE_SET_TEST_STR: &str = "1|2";
    const RULE_SET_TEST_STR_INVALID: &str = "1|2|3";
    use super::*;
    #[test]
    fn test_rule_set() {
        let rule_set = SafetyRule::try_from(RULE_SET_TEST_STR).unwrap();
        assert_eq!(rule_set.x, 1);
        assert_eq!(rule_set.y, 2);
        let error = SafetyRule::try_from(RULE_SET_TEST_STR_INVALID).unwrap_err();
        assert_eq!(error, "Invalid rule: 1|2|3");
    }
    const RULE_SET_LIST: &str = r#"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13
    "#;
    #[test]
    fn test_rule_set_list() {
        let list = RULE_SET_LIST
            .split('\n')
            .filter_map(|x| SafetyRule::try_from(x).ok())
            .collect::<Vec<SafetyRule>>();
        println!("List: {:?}", list);
        let graph = SafetyGraph::new(&list, &SafetyUpdate { list: vec![47, 53] });
        println!();
        println!("In-degree");
        for (k, v) in &graph.in_degree {
            println!("Key: {}, Value: {:?}", k, v);
        }
    }
    const UPDATE_LIST: &str = r#"
    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47
    "#;
    #[test]
    fn test_update_list() {
        let rule_list = RULE_SET_LIST
            .split('\n')
            .filter_map(|x| SafetyRule::try_from(x).ok())
            .collect::<Vec<SafetyRule>>();
        let graph = SafetyGraph::new(&rule_list, &SafetyUpdate { list: vec![47, 53] });
        println!();
        let mut graphs_in_degree = graph.in_degree.iter().collect::<Vec<(&u32, &usize)>>();
        graphs_in_degree.sort_by(|a, b| a.1.cmp(b.1));
        for (k, v) in graphs_in_degree {
            println!("Key: {}, Value: {:?}", k, v);
        }
        let list = UPDATE_LIST.split('\n').collect::<Vec<&str>>();
        let mut total = 0;
        let mut unordered_total = 0;
        for update in list {
            let trimmed_update = update.trim();
            let update_list = trimmed_update
                .split(',')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>();
            if update_list.is_empty() {
                continue;
            }
            let mut lastlevel_of_nodes = None;
            let mut in_order = true;
            update_list.iter().for_each(|node| {
                if let Some(node_level) = graph.in_degree.get(node) {
                    match lastlevel_of_nodes {
                        Some(last_level) => {
                            if node_level > last_level {
                                lastlevel_of_nodes = Some(node_level);
                            } else {
                                in_order = false;
                            }
                        }
                        None => {
                            lastlevel_of_nodes = Some(node_level);
                        }
                    }
                }
            });
            println!();
            println!("Update: {:?} is ordered: {}", update_list, in_order);
            if in_order {
                let middle_point = update_list.len() / 2;
                let middle_number = update_list[middle_point];
                //println!("Middle number: {}", middle_number);
                total += middle_number;
            } else {
                let mut nodes_with_level = update_list
                    .iter()
                    .filter_map(|node| graph.in_degree.get(node).map(|level| (*node, *level)))
                    .collect::<Vec<(u32, usize)>>();
                nodes_with_level.sort_by(|a, b| a.1.cmp(&b.1));
                println!("Nodes with level: {:?}", nodes_with_level);
                let sorted_nodes = nodes_with_level
                    .iter()
                    .map(|(node, _)| *node)
                    .collect::<Vec<u32>>();
                let middle_point = (sorted_nodes.len()) / 2;
                let middle_number = sorted_nodes[middle_point];
                println!("Middle number: {}", middle_number);
                unordered_total += middle_number;
            }
            println!("Total: {}", total);
        }
        assert_eq!(total, 143);
        assert_eq!(unordered_total, 123);
    }
}
