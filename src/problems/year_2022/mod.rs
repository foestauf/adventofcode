use std::io;

mod problem_1;
mod problem_2;
mod problem_3;
mod problem_4;
mod problem_5;

pub fn year_2022() {
    let mut problem_number = String::new();
    println!("Enter a problem number:");
    io::stdin()
        .read_line(&mut problem_number)
        .expect("Failed to read line");

    let problem_number: u32 = problem_number.trim().parse().expect("Please Type a number");

    match problem_number {
        1 => problem_1::solve(),
        2 => problem_2::solve(),
        3 => problem_3::solve(),
        4 => problem_4::solve(),
        5 => problem_5::solve(),
        _ => println!("Invalid problem number"),
    }
}
