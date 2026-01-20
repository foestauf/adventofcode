use std::collections::{HashSet, VecDeque};

const NEIGHBORS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn parse_input(input: &str) -> HashSet<(i32, i32, i32)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i32> = line.split(',').map(|n| n.parse().unwrap()).collect();
            (parts[0], parts[1], parts[2])
        })
        .collect()
}

fn solve_part1(cubes: &HashSet<(i32, i32, i32)>) -> usize {
    let mut surface_area = 0;
    for &(x, y, z) in cubes {
        for (dx, dy, dz) in NEIGHBORS {
            let neighbor = (x + dx, y + dy, z + dz);
            if !cubes.contains(&neighbor) {
                surface_area += 1;
            }
        }
    }
    surface_area
}

fn solve_part2(cubes: &HashSet<(i32, i32, i32)>) -> usize {
    // Find bounding box with 1 unit padding
    let min_x = cubes.iter().map(|c| c.0).min().unwrap() - 1;
    let max_x = cubes.iter().map(|c| c.0).max().unwrap() + 1;
    let min_y = cubes.iter().map(|c| c.1).min().unwrap() - 1;
    let max_y = cubes.iter().map(|c| c.1).max().unwrap() + 1;
    let min_z = cubes.iter().map(|c| c.2).min().unwrap() - 1;
    let max_z = cubes.iter().map(|c| c.2).max().unwrap() + 1;

    // BFS from outside corner to find all exterior air
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let start = (min_x, min_y, min_z);
    queue.push_back(start);
    visited.insert(start);

    let mut exterior_surface = 0;

    while let Some((x, y, z)) = queue.pop_front() {
        for (dx, dy, dz) in NEIGHBORS {
            let neighbor = (x + dx, y + dy, z + dz);
            let (nx, ny, nz) = neighbor;

            // Check bounds
            if nx < min_x || nx > max_x || ny < min_y || ny > max_y || nz < min_z || nz > max_z {
                continue;
            }

            if cubes.contains(&neighbor) {
                // This neighbor is a lava cube - count the face
                exterior_surface += 1;
            } else if !visited.contains(&neighbor) {
                // This is air we haven't visited - add to queue
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    exterior_surface
}

pub fn solve() {
    let input = include_str!("input.txt");
    let cubes = parse_input(input);

    let part1 = solve_part1(&cubes);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&cubes);
    println!("Part 2: {}", part2);
}
