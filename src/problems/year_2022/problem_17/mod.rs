use std::collections::{HashMap, HashSet};

type Rock = Vec<(i64, i64)>;

fn get_rocks() -> Vec<Rock> {
    vec![
        // ####
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        // .#.
        // ###
        // .#.
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        // ..#
        // ..#
        // ###
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        // #
        // #
        // #
        // #
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        // ##
        // ##
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ]
}

fn can_move(rock: &Rock, x: i64, y: i64, chamber: &HashSet<(i64, i64)>) -> bool {
    for &(dx, dy) in rock {
        let nx = x + dx;
        let ny = y + dy;
        if nx < 0 || nx >= 7 || ny <= 0 || chamber.contains(&(nx, ny)) {
            return false;
        }
    }
    true
}

fn place_rock(rock: &Rock, x: i64, y: i64, chamber: &mut HashSet<(i64, i64)>, height: &mut i64) {
    for &(dx, dy) in rock {
        let nx = x + dx;
        let ny = y + dy;
        chamber.insert((nx, ny));
        *height = (*height).max(ny);
    }
}

fn get_top_profile(chamber: &HashSet<(i64, i64)>, height: i64, depth: i64) -> Vec<u8> {
    let mut profile = Vec::new();
    for row in (height - depth + 1).max(1)..=height {
        let mut byte: u8 = 0;
        for col in 0..7 {
            if chamber.contains(&(col, row)) {
                byte |= 1 << col;
            }
        }
        profile.push(byte);
    }
    profile
}

fn simulate(jets: &[char], num_rocks: usize) -> i64 {
    let rocks = get_rocks();
    let mut chamber: HashSet<(i64, i64)> = HashSet::new();
    let mut height: i64 = 0;
    let mut jet_idx = 0;

    for rock_num in 0..num_rocks {
        let rock = &rocks[rock_num % 5];
        let mut x: i64 = 2;
        let mut y: i64 = height + 4;

        loop {
            // Jet push
            let jet = jets[jet_idx % jets.len()];
            jet_idx += 1;
            let new_x = if jet == '<' { x - 1 } else { x + 1 };
            if can_move(rock, new_x, y, &chamber) {
                x = new_x;
            }

            // Fall down
            if can_move(rock, x, y - 1, &chamber) {
                y -= 1;
            } else {
                place_rock(rock, x, y, &mut chamber, &mut height);
                break;
            }
        }
    }

    height
}

fn simulate_with_cycle_detection(jets: &[char], target: u64) -> i64 {
    let rocks = get_rocks();
    let mut chamber: HashSet<(i64, i64)> = HashSet::new();
    let mut height: i64 = 0;
    let mut jet_idx: usize = 0;

    // State cache: (jet_index mod jets.len(), rock_index mod 5, top_profile) -> (rocks_placed, height)
    let mut seen: HashMap<(usize, usize, Vec<u8>), (u64, i64)> = HashMap::new();

    let mut rock_num: u64 = 0;
    let mut added_height: i64 = 0;

    while rock_num < target {
        let rock_idx = (rock_num % 5) as usize;
        let rock = &rocks[rock_idx];
        let jet_state = jet_idx % jets.len();

        // Create cycle key after sufficient height
        if height > 30 {
            let profile = get_top_profile(&chamber, height, 30);
            let key = (jet_state, rock_idx, profile);

            if let Some(&(prev_rocks, prev_height)) = seen.get(&key) {
                // Found a cycle!
                let cycle_length = rock_num - prev_rocks;
                let cycle_height = height - prev_height;
                let remaining_rocks = target - rock_num;
                let full_cycles = remaining_rocks / cycle_length;
                let leftover = remaining_rocks % cycle_length;

                added_height = (full_cycles as i64) * cycle_height;
                rock_num = target - leftover;

                // Clear seen to prevent further cycle detection
                seen.clear();
            } else {
                seen.insert(key, (rock_num, height));
            }
        }

        if rock_num >= target {
            break;
        }

        let mut x: i64 = 2;
        let mut y: i64 = height + 4;

        loop {
            // Jet push
            let jet = jets[jet_idx % jets.len()];
            jet_idx += 1;
            let new_x = if jet == '<' { x - 1 } else { x + 1 };
            if can_move(rock, new_x, y, &chamber) {
                x = new_x;
            }

            // Fall down
            if can_move(rock, x, y - 1, &chamber) {
                y -= 1;
            } else {
                place_rock(rock, x, y, &mut chamber, &mut height);
                break;
            }
        }

        rock_num += 1;
    }

    height + added_height
}

pub fn solve() {
    let input = include_str!("input.txt").trim();
    let jets: Vec<char> = input.chars().collect();

    let part1 = simulate(&jets, 2022);
    println!("Part 1: {}", part1);

    let part2 = simulate_with_cycle_detection(&jets, 1_000_000_000_000);
    println!("Part 2: {}", part2);
}
