use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn is_adjacent(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn move_towards(&mut self, target: &Position) {
        if self.x != target.x {
            self.x += (target.x - self.x).signum();
        }
        if self.y != target.y {
            self.y += (target.y - self.y).signum();
        }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Position>,
    visited_positions: HashSet<Position>,
}

impl Rope {
    fn new(num_knots: usize) -> Self {
        let start = Position::new(0, 0);
        Rope {
            knots: vec![start; num_knots],
            visited_positions: HashSet::from([start]),
        }
    }

    fn move_head(&mut self, direction: char, steps: u32) {
        let (dx, dy) = match direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => panic!("Invalid direction: {}", direction),
        };

        for _ in 0..steps {
            // Move the head
            self.knots[0].move_by(dx, dy);

            // Update each knot following the previous one
            for i in 1..self.knots.len() {
                let target = self.knots[i - 1];
                if !self.knots[i].is_adjacent(&target) {
                    self.knots[i].move_towards(&target);
                }
            }

            // Record the position of the last knot
            self.visited_positions.insert(self.knots[self.knots.len() - 1]);
        }
    }
}

fn parse_input(input: &str) -> Vec<(char, u32)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let direction = parts[0].chars().next().unwrap();
            let steps = parts[1].parse().unwrap();
            (direction, steps)
        })
        .collect()
}

fn solve_part1(input_text: &str) -> usize {
    let moves = parse_input(input_text);
    let mut rope = Rope::new(2); // Head and tail only

    for (direction, steps) in moves {
        rope.move_head(direction, steps);
    }

    rope.visited_positions.len()
}

fn solve_part2(input_text: &str) -> usize {
    let moves = parse_input(input_text);
    let mut rope = Rope::new(10); // 10 knots

    for (direction, steps) in moves {
        rope.move_head(direction, steps);
    }

    rope.visited_positions.len()
}

pub fn solve() {
    println!("Problem 9");
    let input_path = PathBuf::from("src/problems/year_2022/problem_9/input.txt");
    let input_text = fs::read_to_string(input_path).expect("Failed to read input file");
    let result1 = solve_part1(&input_text);
    let result2 = solve_part2(&input_text);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
} 