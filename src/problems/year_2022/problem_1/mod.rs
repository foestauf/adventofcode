use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("Problem 1");
    let file = File::open("src/problems/year_2022/problem_1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut elf_calories = Vec::new();
    let mut elf_total_calories = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            // End of elf's food items, calculate total calories
            let total_calories: i32 = elf_calories.iter().sum();
            elf_total_calories.push(total_calories);
            elf_calories.clear();
        } else {
            // Parse food items for current elf
            let calories: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            elf_calories.extend(calories);
        }
    }

    // Find elf with highest total calories
    let (highest_elf, highest_calories) = elf_total_calories
        .iter()
        .enumerate()
        .max_by_key(|(_, &calories)| calories)
        .unwrap();

    let mut sorted_elf_total_calories = elf_total_calories.clone();
    sorted_elf_total_calories.sort();
    sorted_elf_total_calories.reverse(); // Sort in descending order

    let top_3_elves_total_calories: i32 = sorted_elf_total_calories.iter().take(3).sum();

    println!(
        "Elf {} has the most food with {} calories",
        highest_elf + 1,
        highest_calories
    );
    println!(
        "The sum of the total calories for the top 3 elves is {}",
        top_3_elves_total_calories
    );
}
