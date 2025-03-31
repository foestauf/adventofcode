use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "noop" => Instruction::Noop,
                "addx" => Instruction::Addx(parts[1].parse().unwrap()),
                _ => panic!("Unknown instruction: {}", parts[0]),
            }
        })
        .collect()
}

fn is_measurement_cycle(cycle: i32) -> bool {
    cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0)
}

fn solve_part1_old(input_text: &str) -> i32 {
    let instructions = parse_input(input_text);
    let mut x = 1;
    let mut cycle = 1;
    let mut sum = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
                    let signal_strength = cycle * x;
                    println!("Cycle {}: X = {}, Signal Strength = {}", cycle, x, signal_strength);
                    sum += signal_strength;
                }
                cycle += 1;
            }
            Instruction::Addx(value) => {
                // First cycle
                if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
                    let signal_strength = cycle * x;
                    println!("Cycle {}: X = {}, Signal Strength = {}", cycle, x, signal_strength);
                    sum += signal_strength;
                }
                cycle += 1;

                // Second cycle
                if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
                    let signal_strength = cycle * x;
                    println!("Cycle {}: X = {}, Signal Strength = {}", cycle, x, signal_strength);
                    sum += signal_strength;
                }
                cycle += 1;
                x += value;
            }
        }
    }

    sum
}

fn solve_part1(input_text: &str) -> i32 {
    let instructions = parse_input(input_text);
    if instructions.is_empty() && !input_text.trim().is_empty() {
        eprintln!("Warning: Input file was read but parsed into zero instructions!");
    } else if instructions.is_empty() {
         eprintln!("Warning: No instructions found (input empty or parsing failed silently?)");
    }

    let mut x: i32 = 1;
    let mut cycle: i32 = 0; // Start cycle counter before the first cycle begins
    let mut sum: i32 = 0;
    let mut instruction_iter = instructions.into_iter(); // Use into_iter to consume instructions

    let mut current_instruction_cycles_left = 0;
    let mut value_to_add_later = 0; // Value from addx to apply when its cycles complete

    // Loop indefinitely, driven by cycles, until instructions run out and ongoing operations finish
    loop {
        cycle += 1; // Advance to the current cycle number (1, 2, 3, ...)

        // --- Check Signal Strength ---
        // Check *during* this cycle (using X value from the start of the cycle)
        if is_measurement_cycle(cycle) {
            let signal_strength = cycle * x;
            println!("Cycle {}: X = {}, Signal Strength = {}", cycle, x, signal_strength);
            sum += signal_strength;
        }

        // --- Process Instruction Timing ---
        if current_instruction_cycles_left > 0 {
            // An instruction (must be addx) is ongoing. Decrement its remaining time.
            current_instruction_cycles_left -= 1;
            if current_instruction_cycles_left == 0 {
                // This addx instruction just finished *at the end* of this cycle.
                // Apply the change to X, which will be effective at the *start* of the next cycle.
                x += value_to_add_later;
            }
        } else {
            // No instruction ongoing, try to fetch the next one.
            if let Some(instruction) = instruction_iter.next() {
                match instruction {
                    Instruction::Noop => {
                        // noop starts and finishes in this single cycle.
                        // No change to X, no cycles left.
                        current_instruction_cycles_left = 0;
                    }
                    Instruction::Addx(value) => {
                        // addx starts in this cycle, takes 2 cycles total.
                        // 1 cycle remaining after this one.
                        current_instruction_cycles_left = 1;
                        // Store the value to add *after* the remaining cycle finishes.
                        value_to_add_later = value;
                    }
                }
            } else {
                // No instructions left, and no instruction ongoing. We are done.
                break;
            }
        }
    }

    sum
}

fn solve_part2(input_text: &str) -> String {
    let instructions = parse_input(input_text);
    let mut x: i32 = 1;  // X register controls sprite position
    let mut cycle: i32 = 0;
    let mut screen = String::with_capacity(246);  // 6 rows of 40 chars + newlines
    let mut instruction_iter = instructions.into_iter();
    let mut current_instruction_cycles_left = 0;
    let mut value_to_add_later = 0;

    loop {
        cycle += 1;
        
        // Calculate CRT position (0-39 for each row)
        let crt_pos = (cycle - 1) % 40;
        
        // Draw pixel based on sprite position
        // Sprite is 3 pixels wide and centered on X
        if (x - 1..=x + 1).contains(&crt_pos) {
            screen.push('#');
        } else {
            screen.push('.');
        }
        
        // Add newline after every 40 pixels
        if crt_pos == 39 {
            screen.push('\n');
        }

        // Process instruction timing (same as part 1)
        if current_instruction_cycles_left > 0 {
            current_instruction_cycles_left -= 1;
            if current_instruction_cycles_left == 0 {
                x += value_to_add_later;
            }
        } else {
            if let Some(instruction) = instruction_iter.next() {
                match instruction {
                    Instruction::Noop => {
                        current_instruction_cycles_left = 0;
                    }
                    Instruction::Addx(value) => {
                        current_instruction_cycles_left = 1;
                        value_to_add_later = value;
                    }
                }
            } else {
                break;
            }
        }
        
        // Stop after 240 cycles (6 rows of 40 pixels)
        if cycle >= 240 {
            break;
        }
    }

    screen
}

pub fn solve() {
    println!("Problem 10");
    let input_path = PathBuf::from("src/problems/year_2022/problem_10/input.txt");
    let input_text = fs::read_to_string(input_path).expect("Failed to read input file");
    let result1 = solve_part1(&input_text);
    println!("Part 1: {}", result1);
    
    let result2 = solve_part2(&input_text);
    println!("Part 2:\n{}", result2);
} 