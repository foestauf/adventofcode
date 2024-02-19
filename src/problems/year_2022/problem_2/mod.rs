use std::fs::File;
use std::io::BufRead;

pub fn calculate_score(val1: &str, val2: &str) -> i32 {
    let choice_map = |c: &str| match c {
        "A" | "X" => ("Rock", 1),
        "B" | "Y" => ("Paper", 2),
        "C" | "Z" => ("Scissors", 3),
        _ => panic!("Invalid choice"),
    };

    let (choice1, _ ) = choice_map(val1);
    let (choice2, choice_score) = choice_map(val2);

    let outcome_score = match (choice1, choice2) {
        ("Rock", "Scissors") | ("Paper", "Rock") | ("Scissors", "Paper") => 0,
        ("Rock", "Paper") | ("Paper", "Scissors") | ("Scissors", "Rock") => 6,
        _ => 3,
    };

    choice_score + outcome_score
}

pub fn solve() {
    println!("Problem 2");
    let file = File::open("src/problems/year_2022/problem_2/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut player_score = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let choices: Vec<&str> = line.split_whitespace().collect();
        let score = calculate_score(choices[0], choices[1]);
        player_score += score;
    }

    println!("Player score: {}", player_score);
}