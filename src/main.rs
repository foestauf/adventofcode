mod problems;

use std::io;
use std::time::Instant;

fn main() {
    println!("Project Euler");
    println!("Available years:");
    for (i, year) in problems::AVAILABLE_YEARS.iter().enumerate() {
        println!("  {}) {}", i + 1, year);
    }
    println!("Select a year (1-{}):", problems::AVAILABLE_YEARS.len());

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let year = match input.trim().parse::<usize>() {
        Ok(n) if (1..=problems::AVAILABLE_YEARS.len()).contains(&n) => {
            problems::AVAILABLE_YEARS[n - 1]
        }
        _ => {
            println!("Invalid selection");
            return;
        }
    };

    let start_time = Instant::now();

    problems::select_year(year);

    let elapsed = start_time.elapsed(); // Calculate the elapsed time

    println!("Done! It took {:?} to solve the problem.", elapsed);
}
