use std::collections::HashMap;

fn main() {
    first_puzzle();
    second_puzzle();
}
fn second_puzzle() {
    let mut id_list_a = vec![];
    let mut id_list_b = vec![];
    let input_file = std::fs::read_to_string("days/one/input.txt").unwrap();
    let lines: Vec<&str> = input_file.lines().collect();
    lines.iter().for_each(|line| {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let a = split_line[0].parse::<i32>().unwrap();
        let b = split_line[1].parse::<i32>().unwrap();

        id_list_a.push(a);
        id_list_b.push(b);
    });
    let mut hash_map_a = HashMap::new();
    let mut hash_map_b = HashMap::new();
    id_list_a
        .iter()
        .for_each(|id| match hash_map_a.get_mut(id) {
            Some(value) => {
                *value += 1;
            }
            None => {
                hash_map_a.insert(id, 1);
            }
        });
    id_list_b
        .iter()
        .for_each(|id| match hash_map_b.get_mut(id) {
            Some(value) => {
                *value += 1;
            }
            None => {
                hash_map_b.insert(id, 1);
            }
        });
    println!("Hash map A: {:?}", hash_map_a.len());
    println!("Hash map B: {:?}", hash_map_b.len());
    let total_similarity = hash_map_b.iter().filter_map(|(item_key, item_times_seen)| {
        let value = hash_map_a.get(item_key)?;
        let similarity = item_times_seen * value * *item_key;
        Some(similarity)
    }).sum::<i32>();
    println!("Similarity: {:?}", total_similarity);
}
fn first_puzzle() {
    let mut id_list_a = vec![];
    let mut id_list_b = vec![];

    let input_file = std::fs::read_to_string("days/one/input.txt").unwrap();
    let lines: Vec<&str> = input_file.lines().collect();
    lines.iter().for_each(|line| {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let a = split_line[0].parse::<i32>().unwrap();
        let b = split_line[1].parse::<i32>().unwrap();

        id_list_a.push(a);
        id_list_b.push(b);
    });
    id_list_a.sort();
    id_list_b.sort();
    println!(
        "List LEN A: {:?} list first {} list last {}",
        id_list_a.len(),
        id_list_a.first().unwrap(),
        id_list_a.last().unwrap()
    );
    println!(
        "List LEN B: {:?} list first {} list last {}",
        id_list_b.len(),
        id_list_b.first().unwrap(),
        id_list_b.last().unwrap()
    );
    let sum = id_list_a
        .iter()
        .enumerate()
        .map(|(position_in_list, id_number_a)| {
            let id_number_b = id_list_b[position_in_list];
            let distance_between = (id_number_a - id_number_b).abs();
            distance_between
        })
        .sum::<i32>();
    println!("Total distance between lists: {:?}", sum);
}
