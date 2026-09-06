use std::fs;

fn mix(numbers: &[i64], rounds: usize) -> Vec<i64> {
    let n = numbers.len();
    let modulus = (n as i64) - 1;

    // indices[i] = current position of the element that started at original index i
    let mut indices: Vec<usize> = (0..n).collect();

    for _ in 0..rounds {
        for original in 0..n {
            let pos = indices[original];
            let value = numbers[original];

            let mut new_pos = (pos as i64 + value).rem_euclid(modulus) as usize;
            // After removal there are modulus = n-1 slots; positions 0 and n-1 are adjacent
            // in the circular list. AoC's convention places wrapped elements at the end.
            if new_pos == 0 {
                new_pos = (n - 1) as usize;
            }

            if new_pos == pos {
                continue;
            }

            if new_pos < pos {
                for idx in indices.iter_mut() {
                    if *idx >= new_pos && *idx < pos {
                        *idx += 1;
                    }
                }
            } else {
                for idx in indices.iter_mut() {
                    if *idx > pos && *idx <= new_pos {
                        *idx -= 1;
                    }
                }
            }
            indices[original] = new_pos;
        }
    }

    let mut result = vec![0i64; n];
    for (original, &pos) in indices.iter().enumerate() {
        result[pos] = numbers[original];
    }
    result
}

fn grove_coordinates(mixed: &[i64]) -> i64 {
    let n = mixed.len();
    let zero_pos = mixed.iter().position(|&x| x == 0).expect("no zero in input");
    let a = mixed[(zero_pos + 1000) % n];
    let b = mixed[(zero_pos + 2000) % n];
    let c = mixed[(zero_pos + 3000) % n];
    a + b + c
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse().expect("invalid integer"))
        .collect()
}

fn solve_part1(numbers: &[i64]) -> i64 {
    let mixed = mix(numbers, 1);
    grove_coordinates(&mixed)
}

fn solve_part2(numbers: &[i64]) -> i64 {
    const KEY: i64 = 811_589_153;
    let scaled: Vec<i64> = numbers.iter().map(|&x| x * KEY).collect();
    let mixed = mix(&scaled, 10);
    grove_coordinates(&mixed)
}

pub fn solve() {
    let input = fs::read_to_string("src/problems/year_2022/problem_20/input.txt")
        .expect("Could not read input file");

    let numbers = parse_input(&input);

    let part1 = solve_part1(&numbers);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&numbers);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1\n2\n-3\n3\n-2\n0\n4\n";

    #[test]
    fn sample_part1() {
        let numbers = parse_input(SAMPLE);
        assert_eq!(solve_part1(&numbers), 3);
    }

    #[test]
    fn sample_part2() {
        let numbers = parse_input(SAMPLE);
        assert_eq!(solve_part2(&numbers), 1623178306);
    }
}
