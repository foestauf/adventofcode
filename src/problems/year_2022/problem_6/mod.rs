use std::fs;
use std::path::PathBuf;

fn find_marker(input: &str, marker_size: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    
    for i in 0..chars.len() - marker_size + 1 {
        let window = &chars[i..i + marker_size];
        let mut unique = true;
        
        // Check if all characters in the window are unique
        for j in 0..marker_size {
            for k in j + 1..marker_size {
                if window[j] == window[k] {
                    unique = false;
                    break;
                }
            }
            if !unique {
                break;
            }
        }
        
        if unique {
            return i + marker_size;
        }
    }
    
    panic!("No marker found");
}

fn solve_part1(input_text: &str) -> usize {
    find_marker(input_text.trim(), 4)
}

fn solve_part2(input_text: &str) -> usize {
    find_marker(input_text.trim(), 14)
}

pub fn solve() {
    println!("Problem 6");
    let input_path = PathBuf::from("src/problems/year_2022/problem_6/input.txt");
    let input_text = fs::read_to_string(input_path).expect("Failed to read input file");
    let result1 = solve_part1(&input_text);
    let result2 = solve_part2(&input_text);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
} 