use std::collections::{HashMap, VecDeque};

pub fn solve() {
    let input = include_str!("input.txt");

    let (valves, start_idx) = parse_input(input);
    let valuable_valves = get_valuable_valves(&valves);
    let distances = compute_distances(&valves, start_idx, &valuable_valves);

    println!("Problem 16 - Part 1");
    let part1 = solve_part1(&valves, &valuable_valves, &distances, start_idx, 30);
    println!("{}", part1);

    println!("Problem 16 - Part 2");
    let part2 = solve_part2(&valves, &valuable_valves, &distances, start_idx, 26);
    println!("{}", part2);
}

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: u32,
    tunnels: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Valve>, usize) {
    let mut name_to_idx: HashMap<&str, usize> = HashMap::new();
    let mut valve_data: Vec<(&str, u32, Vec<&str>)> = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let parts: Vec<&str> = line.split(&[' ', '=', ';', ',']).collect();
        let name = parts[1];
        let flow_rate: u32 = parts[5].parse().unwrap();

        let tunnels: Vec<&str> = parts[11..]
            .iter()
            .filter(|s| !s.is_empty())
            .copied()
            .collect();

        name_to_idx.insert(name, idx);
        valve_data.push((name, flow_rate, tunnels));
    }

    let valves: Vec<Valve> = valve_data
        .iter()
        .map(|(_, flow_rate, tunnels)| {
            Valve {
                flow_rate: *flow_rate,
                tunnels: tunnels.iter().map(|t| name_to_idx[t]).collect(),
            }
        })
        .collect();

    let start_idx = name_to_idx["AA"];
    (valves, start_idx)
}

fn get_valuable_valves(valves: &[Valve]) -> Vec<usize> {
    valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(i, _)| i)
        .collect()
}

fn compute_distances(
    valves: &[Valve],
    start_idx: usize,
    valuable_valves: &[usize],
) -> HashMap<(usize, usize), u32> {
    let mut distances = HashMap::new();

    // Compute distances from start and from each valuable valve
    let sources: Vec<usize> = std::iter::once(start_idx)
        .chain(valuable_valves.iter().copied())
        .collect();

    for &source in &sources {
        let dist = bfs(valves, source);
        for &target in valuable_valves {
            if source != target {
                distances.insert((source, target), dist[target]);
            }
        }
    }

    distances
}

fn bfs(valves: &[Valve], start: usize) -> Vec<u32> {
    let mut dist = vec![u32::MAX; valves.len()];
    let mut queue = VecDeque::new();

    dist[start] = 0;
    queue.push_back(start);

    while let Some(curr) = queue.pop_front() {
        for &next in &valves[curr].tunnels {
            if dist[next] == u32::MAX {
                dist[next] = dist[curr] + 1;
                queue.push_back(next);
            }
        }
    }

    dist
}

fn solve_part1(
    valves: &[Valve],
    valuable_valves: &[usize],
    distances: &HashMap<(usize, usize), u32>,
    start_idx: usize,
    time_limit: u32,
) -> u32 {
    let mut best = 0;
    let mut memo: HashMap<(usize, u64, u32), u32> = HashMap::new();

    dfs_part1(
        valves,
        valuable_valves,
        distances,
        start_idx,
        0,
        time_limit,
        0,
        &mut best,
        &mut memo,
    );

    best
}

fn dfs_part1(
    valves: &[Valve],
    valuable_valves: &[usize],
    distances: &HashMap<(usize, usize), u32>,
    current: usize,
    opened: u64,
    time_left: u32,
    pressure: u32,
    best: &mut u32,
    memo: &mut HashMap<(usize, u64, u32), u32>,
) {
    *best = (*best).max(pressure);

    // Memoization check
    let key = (current, opened, time_left);
    if let Some(&cached) = memo.get(&key) {
        if cached >= pressure {
            return;
        }
    }
    memo.insert(key, pressure);

    for (i, &valve_idx) in valuable_valves.iter().enumerate() {
        let mask = 1u64 << i;
        if opened & mask != 0 {
            continue;
        }

        let dist = distances.get(&(current, valve_idx)).copied().unwrap_or(u32::MAX);
        let time_needed = dist + 1; // travel + open

        if time_needed < time_left {
            let new_time = time_left - time_needed;
            let new_pressure = pressure + valves[valve_idx].flow_rate * new_time;

            dfs_part1(
                valves,
                valuable_valves,
                distances,
                valve_idx,
                opened | mask,
                new_time,
                new_pressure,
                best,
                memo,
            );
        }
    }
}

fn solve_part2(
    valves: &[Valve],
    valuable_valves: &[usize],
    distances: &HashMap<(usize, usize), u32>,
    start_idx: usize,
    time_limit: u32,
) -> u32 {
    // Compute best pressure for each possible subset of valves
    let mut best_for_mask: HashMap<u64, u32> = HashMap::new();

    compute_all_subsets(
        valves,
        valuable_valves,
        distances,
        start_idx,
        0,
        time_limit,
        0,
        &mut best_for_mask,
    );

    // Find best combination where human and elephant have disjoint valve sets
    let mut best = 0;
    let all_masks: Vec<(u64, u32)> = best_for_mask.iter().map(|(&k, &v)| (k, v)).collect();

    for i in 0..all_masks.len() {
        for j in i..all_masks.len() {
            let (mask1, score1) = all_masks[i];
            let (mask2, score2) = all_masks[j];

            // Check if disjoint (no valve is opened by both)
            if mask1 & mask2 == 0 {
                best = best.max(score1 + score2);
            }
        }
    }

    best
}

fn compute_all_subsets(
    valves: &[Valve],
    valuable_valves: &[usize],
    distances: &HashMap<(usize, usize), u32>,
    current: usize,
    opened: u64,
    time_left: u32,
    pressure: u32,
    best_for_mask: &mut HashMap<u64, u32>,
) {
    // Record best pressure for this subset of opened valves
    let entry = best_for_mask.entry(opened).or_insert(0);
    *entry = (*entry).max(pressure);

    for (i, &valve_idx) in valuable_valves.iter().enumerate() {
        let mask = 1u64 << i;
        if opened & mask != 0 {
            continue;
        }

        let dist = distances.get(&(current, valve_idx)).copied().unwrap_or(u32::MAX);
        let time_needed = dist + 1;

        if time_needed < time_left {
            let new_time = time_left - time_needed;
            let new_pressure = pressure + valves[valve_idx].flow_rate * new_time;

            compute_all_subsets(
                valves,
                valuable_valves,
                distances,
                valve_idx,
                opened | mask,
                new_time,
                new_pressure,
                best_for_mask,
            );
        }
    }
}
