use std::fs;
use std::path::PathBuf;
use std::collections::{VecDeque, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct HeightMap {
    heights: Vec<Vec<u8>>,
    start: Point,
    end: Point,
}

fn parse_input(input: &str) -> HeightMap {
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    
    let heights: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        'S' => {
                            start = Point { x, y };
                            0 // 'a' - 'a'
                        }
                        'E' => {
                            end = Point { x, y };
                            25 // 'z' - 'a'
                        }
                        c => c as u8 - b'a',
                    }
                })
                .collect()
        })
        .collect();

    HeightMap { heights, start, end }
}

fn get_neighbors(point: Point, height_map: &HeightMap) -> Vec<Point> {
    let mut neighbors = Vec::new();
    let current_height = height_map.heights[point.y][point.x];
    
    // Check all four directions
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    for &(dx, dy) in &directions {
        let new_x = point.x as i32 + dx;
        let new_y = point.y as i32 + dy;
        
        // Check bounds
        if new_x < 0 || new_y < 0 || 
           new_x >= height_map.heights[0].len() as i32 || 
           new_y >= height_map.heights.len() as i32 {
            continue;
        }
        
        let new_x = new_x as usize;
        let new_y = new_y as usize;
        let new_height = height_map.heights[new_y][new_x];
        
        // Can only move up one level
        if new_height <= current_height + 1 {
            neighbors.push(Point { x: new_x, y: new_y });
        }
    }
    
    neighbors
}

fn bfs(start: Point, end: Point, height_map: &HeightMap) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut distances = vec![vec![usize::MAX; height_map.heights[0].len()]; height_map.heights.len()];
    
    queue.push_back(start);
    visited.insert(start);
    distances[start.y][start.x] = 0;
    
    while let Some(current) = queue.pop_front() {
        if current == end {
            return Some(distances[current.y][current.x]);
        }
        
        for neighbor in get_neighbors(current, height_map) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
                distances[neighbor.y][neighbor.x] = distances[current.y][current.x] + 1;
            }
        }
    }
    
    None
}

fn solve_part1(input_text: &str) -> usize {
    let height_map = parse_input(input_text);
    bfs(height_map.start, height_map.end, &height_map)
        .expect("No path found to the end")
}

fn solve_part2(input_text: &str) -> usize {
    let height_map = parse_input(input_text);
    let mut min_steps = usize::MAX;
    
    // Find all starting points at height 0 (a)
    for y in 0..height_map.heights.len() {
        for x in 0..height_map.heights[0].len() {
            if height_map.heights[y][x] == 0 {
                if let Some(steps) = bfs(Point { x, y }, height_map.end, &height_map) {
                    min_steps = min_steps.min(steps);
                }
            }
        }
    }
    
    min_steps
}

pub fn solve() {
    println!("Problem 12");
    let input_path = PathBuf::from("src/problems/year_2022/problem_12/input.txt");
    let input_text = fs::read_to_string(input_path).expect("Failed to read input file");
    
    let result1 = solve_part1(&input_text);
    println!("Part 1: {}", result1);
    
    let result2 = solve_part2(&input_text);
    println!("Part 2: {}", result2);
} 