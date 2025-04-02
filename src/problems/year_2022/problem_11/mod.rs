use std::fs;
use std::path::PathBuf;
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test: Test,
    inspection_count: i64,
}

#[derive(Debug)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
}

#[derive(Debug)]
struct Test {
    divisor: i64,
    true_target: usize,
    false_target: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_text| {
            let lines: Vec<&str> = monkey_text.lines().collect();
            
            // Parse starting items
            let items = lines[1]
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect();

            // Parse operation
            let operation = {
                let op_line = lines[2].split(": ").nth(1).unwrap();
                let parts: Vec<&str> = op_line.split_whitespace().collect();
                match parts[3] {
                    "+" => Operation::Add(parts[4].parse().unwrap()),
                    "*" => {
                        if parts[4] == "old" {
                            Operation::Square
                        } else {
                            Operation::Multiply(parts[4].parse().unwrap())
                        }
                    }
                    _ => panic!("Unknown operation: {}", parts[3]),
                }
            };

            // Parse test
            let test = {
                let divisor = lines[3]
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();
                let true_target = lines[4]
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();
                let false_target = lines[5]
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();
                Test {
                    divisor,
                    true_target,
                    false_target,
                }
            };

            Monkey {
                items,
                operation,
                test,
                inspection_count: 0,
            }
        })
        .collect()
}

fn apply_operation(operation: &Operation, old: i64) -> i64 {
    match operation {
        Operation::Add(n) => old + n,
        Operation::Multiply(n) => old * n,
        Operation::Square => old * old,
    }
}

fn solve_part1(input_text: &str) -> i64 {
    let mut monkeys = parse_input(input_text);
    let rounds = 20;

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspection_count += 1;
                
                // Apply operation and divide by 3 (part 1)
                let new_worry = apply_operation(&monkeys[i].operation, item) / 3;
                
                // Determine target monkey
                let target = if new_worry % monkeys[i].test.divisor == 0 {
                    monkeys[i].test.true_target
                } else {
                    monkeys[i].test.false_target
                };
                
                // Throw item to target monkey
                monkeys[target].items.push_back(new_worry);
            }
        }
    }

    // Calculate monkey business
    let mut counts: Vec<i64> = monkeys.iter().map(|m| m.inspection_count).collect();
    counts.sort_by(|a, b| b.cmp(a));
    counts[0] * counts[1]
}

fn solve_part2(input_text: &str) -> i64 {
    let mut monkeys = parse_input(input_text);
    let rounds = 10000;
    
    // Calculate the product of all divisors for modulo arithmetic
    let modulus: i64 = monkeys.iter().map(|m| m.test.divisor).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspection_count += 1;
                
                // Apply operation and use modulo arithmetic to keep numbers manageable
                let new_worry = apply_operation(&monkeys[i].operation, item) % modulus;
                
                // Determine target monkey
                let target = if new_worry % monkeys[i].test.divisor == 0 {
                    monkeys[i].test.true_target
                } else {
                    monkeys[i].test.false_target
                };
                
                // Throw item to target monkey
                monkeys[target].items.push_back(new_worry);
            }
        }
    }

    // Calculate monkey business
    let mut counts: Vec<i64> = monkeys.iter().map(|m| m.inspection_count).collect();
    counts.sort_by(|a, b| b.cmp(a));
    counts[0] * counts[1]
}

pub fn solve() {
    println!("Problem 11");
    let input_path = PathBuf::from("src/problems/year_2022/problem_11/input.txt");
    let input_text = fs::read_to_string(input_path).expect("Failed to read input file");
    
    let result1 = solve_part1(&input_text);
    println!("Part 1: {}", result1);
    
    let result2 = solve_part2(&input_text);
    println!("Part 2: {}", result2);
} 