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

fn find_badge_item(rucksacks: &[String]) -> char {
    let mut common_items: HashSet<char> = rucksacks[0].chars().collect();
    
    for rucksack in &rucksacks[1..] {
        let rucksack_items: HashSet<char> = rucksack.chars().collect();
        common_items = common_items.intersection(&rucksack_items).copied().collect();
    }
    
    *common_items.iter().next().expect("No common badge found")
}

pub fn solve() {
    println!("Problem 3");
    let file = File::open("src/problems/year_2022/problem_3/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Part 1
    let mut total_priority = 0;
    for line in &lines {
        let common_item = find_common_item(line);
        let priority = get_item_priority(common_item);
        total_priority += priority;
    }
    println!("Part 1 - Total priority: {}", total_priority);

    // Part 2
    let mut badge_priority = 0;
    for group in lines.chunks(3) {
        let badge = find_badge_item(group);
        let priority = get_item_priority(badge);
        badge_priority += priority;
    }
    println!("Part 2 - Total badge priority: {}", badge_priority);
} 