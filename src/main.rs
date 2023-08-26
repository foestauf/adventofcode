mod problems;

use std::io;
use std::time::Instant;

fn main() {
    println!("Project Euler");
    let mut year_number = String::new();
    println!("Enter a year number:");

    io::stdin()
        .read_line(&mut year_number)
        .expect("Failed to read line");

    let start_time = Instant::now();

    problems::select_year(year_number.trim());

    let elapsed = start_time.elapsed(); // Calculate the elapsed time

    println!("Done! It took {:?} to solve the problem.", elapsed);
}
