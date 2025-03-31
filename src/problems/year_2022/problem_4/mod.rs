use std::fs::File;
use std::io::BufRead;

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        // Two ranges overlap if one range's start is less than or equal to the other range's end
        // and the other range's start is less than or equal to the first range's end
        self.start <= other.end && other.start <= self.end
    }
}

fn parse_range(range_str: &str) -> Range {
    let parts: Vec<&str> = range_str.split('-').collect();
    if parts.len() != 2 {
        panic!("Invalid range format: '{}'", range_str);
    }
    Range {
        start: parts[0].parse().expect(&format!("Invalid start number in range: '{}'", range_str)),
        end: parts[1].parse().expect(&format!("Invalid end number in range: '{}'", range_str)),
    }
}

fn parse_line(line: &str) -> (Range, Range) {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 2 {
        panic!("Invalid line format: '{}'", line);
    }
    (parse_range(parts[0]), parse_range(parts[1]))
}

pub fn solve() {
    println!("Problem 4");
    let file = File::open("src/problems/year_2022/problem_4/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut fully_contained_count = 0;
    let mut overlapping_count = 0;
    let mut line_number = 0;

    for line in reader.lines() {
        line_number += 1;
        let line = line.unwrap();
        
        // Skip empty lines
        if line.trim().is_empty() {
            println!("Skipping empty line {}", line_number);
            continue;
        }
        
        println!("Processing line {}: '{}'", line_number, line);
        
        let (range1, range2) = parse_line(&line);
        
        // Part 1: Check if either range fully contains the other
        if range1.contains(&range2) || range2.contains(&range1) {
            fully_contained_count += 1;
        }

        // Part 2: Check if ranges overlap at all
        if range1.overlaps(&range2) {
            overlapping_count += 1;
        }
    }

    println!("Part 1 - Number of pairs where one range fully contains the other: {}", fully_contained_count);
    println!("Part 2 - Number of pairs where ranges overlap: {}", overlapping_count);
} 