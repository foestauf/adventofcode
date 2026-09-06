use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum Job {
    Number(i64),
    Operation(String, Op, String),
}

fn parse_input(input: &str) -> HashMap<String, Job> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (name, rest) = line.split_once(": ").expect("invalid line");
            let parts: Vec<&str> = rest.split_whitespace().collect();
            let job = if parts.len() == 1 {
                Job::Number(parts[0].parse().expect("invalid number"))
            } else {
                let op = match parts[1] {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => panic!("invalid operator"),
                };
                Job::Operation(parts[0].to_string(), op, parts[2].to_string())
            };
            (name.to_string(), job)
        })
        .collect()
}

fn evaluate(name: &str, monkeys: &HashMap<String, Job>) -> i64 {
    match &monkeys[name] {
        Job::Number(n) => *n,
        Job::Operation(l, op, r) => {
            let lv = evaluate(l, monkeys);
            let rv = evaluate(r, monkeys);
            match op {
                Op::Add => lv + rv,
                Op::Sub => lv - rv,
                Op::Mul => lv * rv,
                Op::Div => lv / rv,
            }
        }
    }
}

fn depends_on(name: &str, target: &str, monkeys: &HashMap<String, Job>) -> bool {
    if name == target {
        return true;
    }
    match &monkeys[name] {
        Job::Number(_) => false,
        Job::Operation(l, _, r) => {
            depends_on(l, target, monkeys) || depends_on(r, target, monkeys)
        }
    }
}

fn solve_for_humn(name: &str, expected: i64, monkeys: &HashMap<String, Job>) -> i64 {
    if name == "humn" {
        return expected;
    }
    match &monkeys[name] {
        Job::Number(_) => panic!("reached non-humn leaf while solving"),
        Job::Operation(l, op, r) => {
            if depends_on(l, "humn", monkeys) {
                let rv = evaluate(r, monkeys);
                let new_expected = match op {
                    Op::Add => expected - rv,
                    Op::Sub => expected + rv,
                    Op::Mul => expected / rv,
                    Op::Div => expected * rv,
                };
                solve_for_humn(l, new_expected, monkeys)
            } else {
                let lv = evaluate(l, monkeys);
                let new_expected = match op {
                    Op::Add => expected - lv,
                    Op::Sub => lv - expected,
                    Op::Mul => expected / lv,
                    Op::Div => lv / expected,
                };
                solve_for_humn(r, new_expected, monkeys)
            }
        }
    }
}

fn solve_part1(monkeys: &HashMap<String, Job>) -> i64 {
    evaluate("root", monkeys)
}

fn solve_part2(monkeys: &HashMap<String, Job>) -> i64 {
    let (l, r) = match &monkeys["root"] {
        Job::Operation(l, _, r) => (l.clone(), r.clone()),
        _ => panic!("root must be an operation"),
    };
    if depends_on(&l, "humn", monkeys) {
        let target = evaluate(&r, monkeys);
        solve_for_humn(&l, target, monkeys)
    } else {
        let target = evaluate(&l, monkeys);
        solve_for_humn(&r, target, monkeys)
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/problems/year_2022/problem_21/input.txt")
        .expect("Could not read input file");

    let monkeys = parse_input(&input);

    let part1 = solve_part1(&monkeys);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&monkeys);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
sjmn: drzm * dbpl
";

    #[test]
    fn sample_part1() {
        let monkeys = parse_input(SAMPLE);
        assert_eq!(solve_part1(&monkeys), 152);
    }

    #[test]
    fn sample_part2() {
        let monkeys = parse_input(SAMPLE);
        assert_eq!(solve_part2(&monkeys), 301);
    }
}
