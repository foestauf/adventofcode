use std::fs;
use std::path::PathBuf;
use serde_json::Value;

/// Represents a packet that can be either a number or a list of packets
#[derive(Debug, Clone)]
enum Packet {
    Number(i64),
    List(Vec<Packet>),
}

impl Packet {
    /// Creates a Packet from a JSON Value
    fn from_value(value: &Value) -> Result<Self, String> {
        match value {
            Value::Number(n) => {
                n.as_i64()
                    .ok_or_else(|| "Invalid number in JSON".to_string())
                    .map(Packet::Number)
            }
            Value::Array(arr) => {
                arr.iter()
                    .map(Packet::from_value)
                    .collect::<Result<Vec<_>, _>>()
                    .map(Packet::List)
            }
            _ => Err("Invalid JSON value type".to_string()),
        }
    }

    /// Compares two packets according to the puzzle rules
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Number(l), Packet::Number(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => {
                for (i, l_item) in l.iter().enumerate() {
                    if i >= r.len() {
                        return std::cmp::Ordering::Greater;
                    }
                    match l_item.compare(&r[i]) {
                        std::cmp::Ordering::Equal => continue,
                        other => return other,
                    }
                }
                l.len().cmp(&r.len())
            }
            (Packet::Number(l), Packet::List(r)) => {
                Packet::List(vec![Packet::Number(*l)]).compare(other)
            }
            (Packet::List(l), Packet::Number(r)) => {
                self.compare(&Packet::List(vec![Packet::Number(*r)]))
            }
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(Packet, Packet)>, String> {
    input
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            let left = lines.next().ok_or("Missing left packet")?;
            let right = lines.next().ok_or("Missing right packet")?;
            
            let left_value: Value = serde_json::from_str(left)
                .map_err(|e| format!("Failed to parse left packet: {}", e))?;
            let right_value: Value = serde_json::from_str(right)
                .map_err(|e| format!("Failed to parse right packet: {}", e))?;
            
            Ok((
                Packet::from_value(&left_value)?,
                Packet::from_value(&right_value)?,
            ))
        })
        .collect()
}

fn solve_part1(input_text: &str) -> Result<usize, String> {
    let pairs = parse_input(input_text)?;
    Ok(pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left.compare(right) == std::cmp::Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum())
}

fn solve_part2(input_text: &str) -> Result<usize, String> {
    // Parse all packets
    let mut packets: Vec<Packet> = input_text
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let value: Value = serde_json::from_str(line)
                .map_err(|e| format!("Failed to parse packet: {}", e))?;
            Packet::from_value(&value)
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Add divider packets
    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    packets.push(divider1.clone());
    packets.push(divider2.clone());

    // Sort packets
    packets.sort_by(|a, b| a.compare(b));

    // Find indices of divider packets
    let pos1 = packets.iter().position(|p| p.compare(&divider1) == std::cmp::Ordering::Equal)
        .ok_or("Divider packet 1 not found")? + 1;
    let pos2 = packets.iter().position(|p| p.compare(&divider2) == std::cmp::Ordering::Equal)
        .ok_or("Divider packet 2 not found")? + 1;

    Ok(pos1 * pos2)
}

pub fn solve() {
    println!("Problem 13");
    let input_path = PathBuf::from("src/problems/year_2022/problem_13/input.txt");
    let input_text = fs::read_to_string(input_path).expect("Failed to read input file");
    
    match solve_part1(&input_text) {
        Ok(result) => println!("Part 1: {}", result),
        Err(e) => eprintln!("Error in part 1: {}", e),
    }
    
    match solve_part2(&input_text) {
        Ok(result) => println!("Part 2: {}", result),
        Err(e) => eprintln!("Error in part 2: {}", e),
    }
} 