use std::fs;

#[derive(Debug, Clone)]
struct Blueprint {
    id: u32,
    ore_robot_cost: u32,
    clay_robot_cost: u32,
    obsidian_robot_cost: (u32, u32), // (ore, clay)
    geode_robot_cost: (u32, u32),    // (ore, obsidian)
}

#[derive(Clone, Copy)]
struct State {
    time: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<u32> = line
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

            Blueprint {
                id: nums[0],
                ore_robot_cost: nums[1],
                clay_robot_cost: nums[2],
                obsidian_robot_cost: (nums[3], nums[4]),
                geode_robot_cost: (nums[5], nums[6]),
            }
        })
        .collect()
}

fn time_to_afford(cost: u32, current: u32, robots: u32) -> Option<u32> {
    if current >= cost {
        Some(0)
    } else if robots == 0 {
        None
    } else {
        Some((cost - current + robots - 1) / robots)
    }
}

fn max_geodes(blueprint: &Blueprint, time_limit: u32) -> u32 {
    let mut best = 0u32;

    // Maximum robots needed (no point building more than max consumption per turn)
    let max_ore_robots = blueprint
        .ore_robot_cost
        .max(blueprint.clay_robot_cost)
        .max(blueprint.obsidian_robot_cost.0)
        .max(blueprint.geode_robot_cost.0);
    let max_clay_robots = blueprint.obsidian_robot_cost.1;
    let max_obsidian_robots = blueprint.geode_robot_cost.1;

    let initial = State {
        time: 0,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
    };

    let mut stack = vec![initial];

    while let Some(state) = stack.pop() {
        let remaining = time_limit - state.time;

        // Upper bound: current geodes + geode robots * remaining + triangular number
        let upper_bound = state.geodes
            + state.geode_robots * remaining
            + (remaining * remaining.saturating_sub(1)) / 2;

        if upper_bound <= best {
            continue;
        }

        best = best.max(state.geodes + state.geode_robots * remaining);

        // Try building each robot type by jumping to the time we can afford it

        // Try building geode robot
        if state.obsidian_robots > 0 {
            let time_ore = time_to_afford(blueprint.geode_robot_cost.0, state.ore, state.ore_robots);
            let time_obs = time_to_afford(blueprint.geode_robot_cost.1, state.obsidian, state.obsidian_robots);

            if let (Some(t1), Some(t2)) = (time_ore, time_obs) {
                let wait = t1.max(t2) + 1; // +1 for build time
                if state.time + wait < time_limit {
                    stack.push(State {
                        time: state.time + wait,
                        ore: state.ore + state.ore_robots * wait - blueprint.geode_robot_cost.0,
                        clay: state.clay + state.clay_robots * wait,
                        obsidian: state.obsidian + state.obsidian_robots * wait - blueprint.geode_robot_cost.1,
                        geodes: state.geodes + state.geode_robots * wait,
                        geode_robots: state.geode_robots + 1,
                        ..state
                    });
                }
            }
        }

        // Try building obsidian robot
        if state.obsidian_robots < max_obsidian_robots && state.clay_robots > 0 {
            let time_ore = time_to_afford(blueprint.obsidian_robot_cost.0, state.ore, state.ore_robots);
            let time_clay = time_to_afford(blueprint.obsidian_robot_cost.1, state.clay, state.clay_robots);

            if let (Some(t1), Some(t2)) = (time_ore, time_clay) {
                let wait = t1.max(t2) + 1;
                if state.time + wait < time_limit {
                    stack.push(State {
                        time: state.time + wait,
                        ore: state.ore + state.ore_robots * wait - blueprint.obsidian_robot_cost.0,
                        clay: state.clay + state.clay_robots * wait - blueprint.obsidian_robot_cost.1,
                        obsidian: state.obsidian + state.obsidian_robots * wait,
                        geodes: state.geodes + state.geode_robots * wait,
                        obsidian_robots: state.obsidian_robots + 1,
                        ..state
                    });
                }
            }
        }

        // Try building clay robot
        if state.clay_robots < max_clay_robots {
            if let Some(t) = time_to_afford(blueprint.clay_robot_cost, state.ore, state.ore_robots) {
                let wait = t + 1;
                if state.time + wait < time_limit {
                    stack.push(State {
                        time: state.time + wait,
                        ore: state.ore + state.ore_robots * wait - blueprint.clay_robot_cost,
                        clay: state.clay + state.clay_robots * wait,
                        obsidian: state.obsidian + state.obsidian_robots * wait,
                        geodes: state.geodes + state.geode_robots * wait,
                        clay_robots: state.clay_robots + 1,
                        ..state
                    });
                }
            }
        }

        // Try building ore robot
        if state.ore_robots < max_ore_robots {
            if let Some(t) = time_to_afford(blueprint.ore_robot_cost, state.ore, state.ore_robots) {
                let wait = t + 1;
                if state.time + wait < time_limit {
                    stack.push(State {
                        time: state.time + wait,
                        ore: state.ore + state.ore_robots * wait - blueprint.ore_robot_cost,
                        clay: state.clay + state.clay_robots * wait,
                        obsidian: state.obsidian + state.obsidian_robots * wait,
                        geodes: state.geodes + state.geode_robots * wait,
                        ore_robots: state.ore_robots + 1,
                        ..state
                    });
                }
            }
        }
    }

    best
}

fn solve_part1(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .map(|bp| {
            let geodes = max_geodes(bp, 24);
            bp.id * geodes
        })
        .sum()
}

fn solve_part2(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|bp| max_geodes(bp, 32))
        .product()
}

pub fn solve() {
    let input = fs::read_to_string("src/problems/year_2022/problem_19/input.txt")
        .expect("Could not read input file");

    let blueprints = parse_input(&input);

    let part1 = solve_part1(&blueprints);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&blueprints);
    println!("Part 2: {}", part2);
}
