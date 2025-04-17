use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    // Example: Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let parts: Vec<_> = line.split([',', ':']).collect();
    let sx = parts[0].split('=').nth(1).unwrap().trim().parse().unwrap();
    let sy = parts[1].split('=').nth(1).unwrap().trim().parse().unwrap();
    let bx = parts[2].split('=').nth(1).unwrap().trim().parse().unwrap();
    let by = parts[3].split('=').nth(1).unwrap().trim().parse().unwrap();
    ((sx, sy), (bx, by))
}

pub fn solve() {
    println!("Problem 15 - Part 1");
    let file = File::open("src/problems/year_2022/problem_15/input.txt").unwrap();
    let reader = BufReader::new(file);
    let target_y = 2_000_000;
    let mut covered: HashSet<i32> = HashSet::new();
    let mut beacons_on_row: HashSet<i32> = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() { continue; }
        let ((sx, sy), (bx, by)) = parse_line(&line);
        let dist = (sx - bx).abs() + (sy - by).abs();
        let dy = (sy - target_y).abs();
        if by == target_y {
            beacons_on_row.insert(bx);
        }
        if dy <= dist {
            let dx = dist - dy;
            for x in (sx - dx)..=(sx + dx) {
                covered.insert(x);
            }
        }
    }
    // Remove any positions where a beacon is present
    let result = covered.len() - beacons_on_row.len();
    println!("Positions where a beacon cannot be present on y={}: {}", target_y, result);
    println!("Problem 15 - Part 2");
    solve_part2();
}

pub fn solve_part2() {
    println!("Problem 15 - Part 2");
    let file = File::open("src/problems/year_2022/problem_15/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sensors = Vec::new();
    let mut dists = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() { continue; }
        let ((sx, sy), (bx, by)) = parse_line(&line);
        let dist = (sx - bx).abs() + (sy - by).abs();
        sensors.push((sx, sy));
        dists.push(dist);
    }
    let max_coord = 4_000_000;
    for (i, &(sx, sy)) in sensors.iter().enumerate() {
        let d = dists[i] + 1;
        for dx in 0..=d {
            let dy = d - dx;
            let candidates = [
                (sx + dx, sy + dy),
                (sx + dx, sy - dy),
                (sx - dx, sy + dy),
                (sx - dx, sy - dy),
            ];
            for &(x, y) in &candidates {
                if x < 0 || x > max_coord || y < 0 || y > max_coord { continue; }
                let mut covered = false;
                for (j, &(osx, osy)) in sensors.iter().enumerate() {
                    let od = dists[j];
                    if (osx - x).abs() + (osy - y).abs() <= od {
                        covered = true;
                        break;
                    }
                }
                if !covered {
                    let freq = (x as i64) * 4_000_000 + (y as i64);
                    println!("Distress beacon at x={}, y={}, tuning frequency: {}", x, y, freq);
                    return;
                }
            }
        }
    }
    println!("No valid position found");
}
