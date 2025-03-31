use std::fs;
use std::path::PathBuf;

type Grid = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn is_visible(grid: &Grid, row: usize, col: usize) -> bool {
    let height = grid[row][col];
    let rows = grid.len();
    let cols = grid[0].len();

    // Check visibility from all four directions
    let directions = [
        (0, 1),  // right
        (0, -1), // left
        (1, 0),  // down
        (-1, 0), // up
    ];

    for &(dr, dc) in &directions {
        let mut r = row as i32;
        let mut c = col as i32;
        let mut visible = true;

        // Move in the current direction until we hit the edge
        while r > 0 && r < (rows - 1) as i32 && c > 0 && c < (cols - 1) as i32 {
            r += dr;
            c += dc;
            if grid[r as usize][c as usize] >= height {
                visible = false;
                break;
            }
        }

        if visible {
            return true;
        }
    }

    false
}

fn get_viewing_distance(grid: &Grid, row: usize, col: usize, dr: i32, dc: i32) -> usize {
    let height = grid[row][col];
    let rows = grid.len();
    let cols = grid[0].len();
    let mut distance = 0;
    let mut r = row as i32;
    let mut c = col as i32;

    // Move in the given direction until we hit the edge or a taller tree
    while r > 0 && r < (rows - 1) as i32 && c > 0 && c < (cols - 1) as i32 {
        r += dr;
        c += dc;
        distance += 1;
        if grid[r as usize][c as usize] >= height {
            break;
        }
    }

    distance
}

fn get_scenic_score(grid: &Grid, row: usize, col: usize) -> usize {
    let directions = [
        (0, 1),  // right
        (0, -1), // left
        (1, 0),  // down
        (-1, 0), // up
    ];

    directions
        .iter()
        .map(|&(dr, dc)| get_viewing_distance(grid, row, col, dr, dc))
        .product()
}

fn solve_part1(input_text: &str) -> usize {
    let grid = parse_input(input_text);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visible_count = 0;

    // Count visible trees
    for row in 0..rows {
        for col in 0..cols {
            if is_visible(&grid, row, col) {
                visible_count += 1;
            }
        }
    }

    visible_count
}

fn solve_part2(input_text: &str) -> usize {
    let grid = parse_input(input_text);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut max_score = 0;

    // Calculate scenic score for each tree
    for row in 0..rows {
        for col in 0..cols {
            let score = get_scenic_score(&grid, row, col);
            max_score = max_score.max(score);
        }
    }

    max_score
}

pub fn solve() {
    println!("Problem 8");
    let input_path = PathBuf::from("src/problems/year_2022/problem_8/input.txt");
    let input_text = fs::read_to_string(input_path).expect("Failed to read input file");
    let result1 = solve_part1(&input_text);
    let result2 = solve_part2(&input_text);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
} 