use std::fs::File;
use std::io::BufRead;

fn get_shape_for_outcome(opponent: &str, outcome: &str) -> &'static str {
    match (opponent, outcome) {
        // To lose
        ("A", "X") => "Scissors",  // Rock beats Scissors
        ("B", "X") => "Rock",      // Paper beats Rock
        ("C", "X") => "Paper",     // Scissors beats Paper
        
        // To draw
        ("A", "Y") => "Rock",
        ("B", "Y") => "Paper",
        ("C", "Y") => "Scissors",
        
        // To win
        ("A", "Z") => "Paper",     // Paper beats Rock
        ("B", "Z") => "Scissors",  // Scissors beats Paper
        ("C", "Z") => "Rock",      // Rock beats Scissors
        
        _ => panic!("Invalid input"),
    }
}

pub fn calculate_score(val1: &str, val2: &str) -> i32 {
    let shape_score = |shape: &str| match shape {
        "Rock" => 1,
        "Paper" => 2,
        "Scissors" => 3,
        _ => panic!("Invalid shape"),
    };

    let outcome_score = match val2 {
        "X" => 0,  // Lose
        "Y" => 3,  // Draw
        "Z" => 6,  // Win
        _ => panic!("Invalid outcome"),
    };

    let my_shape = get_shape_for_outcome(val1, val2);
    shape_score(my_shape) + outcome_score
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