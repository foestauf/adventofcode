use std::fs::File;
use std::io::BufRead;
use std::collections::VecDeque;

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(lines: &[String]) -> Vec<VecDeque<char>> {
    // Find the number of stacks from the last line (contains stack numbers)
    let stack_count = lines.last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    
    let mut stacks = vec![VecDeque::new(); stack_count];
    
    // Process each line except the last one (which contains stack numbers)
    for line in &lines[..lines.len()-1] {
        let chars: Vec<char> = line.chars().collect();
        for (i, chunk) in chars.chunks(4).enumerate() {
            if chunk[1] != ' ' {
                stacks[i].push_back(chunk[1]);
            }
        }
    }
    
    stacks
}

fn parse_move(line: &str) -> Move {
    let parts: Vec<&str> = line.split_whitespace().collect();
    Move {
        quantity: parts[1].parse().unwrap(),
        from: parts[3].parse::<usize>().unwrap() - 1, // Convert to 0-based index
        to: parts[5].parse::<usize>().unwrap() - 1,   // Convert to 0-based index
    }
}

fn apply_move_part1(stacks: &mut Vec<VecDeque<char>>, mov: &Move) {
    for _ in 0..mov.quantity {
        if let Some(crate_) = stacks[mov.from].pop_front() {
            stacks[mov.to].push_front(crate_);
        }
    }
}

fn apply_move_part2(stacks: &mut Vec<VecDeque<char>>, mov: &Move) {
    let mut temp_stack = VecDeque::new();
    
    // Move crates to temporary stack
    for _ in 0..mov.quantity {
        if let Some(crate_) = stacks[mov.from].pop_front() {
            temp_stack.push_front(crate_);
        }
    }
    
    // Move crates from temporary stack to destination
    while let Some(crate_) = temp_stack.pop_front() {
        stacks[mov.to].push_front(crate_);
    }
}

pub fn solve() {
    println!("Problem 5");
    let file = File::open("src/problems/year_2022/problem_5/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Find the line that separates stacks from moves
    let separator_index = lines.iter()
        .position(|line| line.trim().is_empty())
        .unwrap();

    // Parse stacks for part 1
    let mut stacks_part1 = parse_stacks(&lines[..separator_index]);
    // Parse stacks for part 2
    let mut stacks_part2 = parse_stacks(&lines[..separator_index]);
    
    // Parse and apply moves
    for line in &lines[separator_index + 1..] {
        if line.trim().is_empty() {
            continue;
        }
        let mov = parse_move(line);
        apply_move_part1(&mut stacks_part1, &mov);
        apply_move_part2(&mut stacks_part2, &mov);
    }

    // Get top crates for part 1
    let top_crates_part1: String = stacks_part1.iter()
        .map(|stack| stack.front().unwrap())
        .collect();

    // Get top crates for part 2
    let top_crates_part2: String = stacks_part2.iter()
        .map(|stack| stack.front().unwrap())
        .collect();

    println!("Part 1 - Top crates: {}", top_crates_part1);
    println!("Part 2 - Top crates: {}", top_crates_part2);
} 