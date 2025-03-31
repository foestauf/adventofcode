use std::fs::File;
use std::io::BufRead;
use std::collections::HashSet;

fn get_item_priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        (item as u32) - ('a' as u32) + 1
    } else if item.is_ascii_uppercase() {
        (item as u32) - ('A' as u32) + 27
    } else {
        panic!("Invalid item type");
    }
}

fn find_common_item(rucksack: &str) -> char {
    let mid = rucksack.len() / 2;
    let compartment1: HashSet<char> = rucksack[..mid].chars().collect();
    let compartment2: HashSet<char> = rucksack[mid..].chars().collect();
    
    *compartment1.intersection(&compartment2)
        .next()
        .expect("No common item found")
}

pub fn solve() {
    println!("Problem 3");
    let file = File::open("src/problems/year_2022/problem_3/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut total_priority = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let common_item = find_common_item(&line);
        let priority = get_item_priority(common_item);
        total_priority += priority;
    }

    println!("Total priority: {}", total_priority);
} 