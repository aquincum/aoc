use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::iter::FromIterator;
use std::str::FromStr;

pub struct Day8;

impl Day for Day8 {
    fn question(&self, input: &str, question: Question) {
        let result = q(input, question);
        if result.is_err() {
            println!("Error: {}", result.unwrap_err());
        } else {
            println!("{}", result.unwrap());
        }
    }

    fn test_data(&self) -> String {
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            .to_string()
        //         "RL
        //
        // AAA = (BBB, CCC)
        // BBB = (DDD, EEE)
        // CCC = (ZZZ, GGG)
        // DDD = (DDD, DDD)
        // EEE = (EEE, EEE)
        // GGG = (GGG, GGG)
        // ZZZ = (ZZZ, ZZZ)"
        //             .to_string()
    }
}

enum Move {
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Move::Left),
            'R' => Ok(Move::Right),
            _ => Err(format!("Invalid move character {}", value)),
        }
    }
}

struct Node {
    id: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, routes) = s
            .split(" = ")
            .collect_tuple()
            .ok_or(format!("can't split {}", s))?;
        let (left, right) = routes
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split(", ")
            .collect_tuple()
            .ok_or(format!("can't split {}", routes))?;
        Ok(Node {
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let (moves, nodes) = input
        .split("\n\n")
        .collect_tuple()
        .ok_or("no two line breaks")?;
    let moves: Result<Vec<Move>, String> = moves.chars().map(|ch| Move::try_from(ch)).collect();
    let moves = moves?;
    let node_vec: Result<Vec<Node>, String> = nodes.lines().map(|l| l.parse::<Node>()).collect();
    let node_vec = node_vec?;
    let node_map: HashMap<String, Node> = HashMap::from_iter(
        node_vec
            .into_iter()
            .map(|node| ((&node.id).to_string(), node)),
    );
    match question {
        Question::First => q1(node_map, moves),
        Question::Second => q2(node_map, moves),
    }
}

fn q1(node_map: HashMap<String, Node>, moves: Vec<Move>) -> Result<u128, String> {
    // let's just
    let mut current = "AAA";
    let mut steps = 0u128;
    let mut current_move = moves.iter().cycle();
    loop {
        let node = node_map
            .get(current)
            .ok_or(format!("node {} not found", current))?;
        let which_way = current_move.next().unwrap();
        current = match which_way {
            Move::Left => &node.left,
            Move::Right => &node.right,
        };
        steps += 1;
        if current == "ZZZ" {
            return Ok(steps);
        }
    }
}

// This is just brute force least common multiple for the patterns discovered in q2_find_series
fn q2(node_map: HashMap<String, Node>, moves: Vec<Move>) -> Result<u128, String> {
    let mut x: u128 = 19783;
    const other_cycles: [u128; 5] = [15989, 18157, 12737, 14363, 19241];
    loop {
        x += 19783;
        let hits = other_cycles.iter().filter(|t| x % **t == 0).count();
        if hits == 5 {
            println!("GOTIT");
            return Ok(x);
        }
        if hits >= 3 {
            println!("{} at {}", hits + 1, x);
        }
    }
}

// Ran this to discover the patterns.
fn q2_find_series(node_map: HashMap<String, Node>, moves: Vec<Move>) -> Result<u128, String> {
    let mut currents = node_map.keys().filter(|id| id.ends_with("A")).collect_vec();
    let mut steps = 0u128;
    for c in currents {
        let mut current_move = moves.iter().enumerate().cycle();
        let mut current = c;
        println!("Starting node: {}", current);
        steps = 0;
        let mut last = None;
        loop {
            let node = node_map
                .get(current)
                .ok_or(format!("node {} not found", current))?;
            let which_way = current_move.next().unwrap();
            current = match which_way.1 {
                Move::Left => &node.left,
                Move::Right => &node.right,
            };
            steps += 1;
            if current.ends_with("Z") {
                println!(
                    "Move {}: at {}. In the cycle: {}. Since last: {}",
                    steps,
                    current,
                    which_way.0,
                    last.map(|l| format!("Since last: {}", steps - l))
                        .unwrap_or("First hit.".to_string())
                );
                last = Some(steps);
            }
            if steps == 200_000 {
                break;
            }
        }
    }
    Ok(9)
}

// horrifically slow and cannot find it
fn q2_slow(node_map: HashMap<String, Node>, moves: Vec<Move>) -> Result<u128, String> {
    let mut currents = node_map.keys().filter(|id| id.ends_with("A")).collect_vec();
    let mut current_move = moves.iter().cycle();
    let mut steps = 0u128;
    loop {
        let nodes: Result<Vec<&Node>, String> = currents
            .iter()
            .map(|id| node_map.get(*id).ok_or(format!("node {} not found", id)))
            .collect();
        let nodes = nodes?;
        let which_way = current_move.next().unwrap();

        currents = nodes
            .iter()
            .map(|n| match which_way {
                Move::Left => &n.left,
                Move::Right => &n.right,
            })
            .collect();
        steps += 1;
        let hits = currents.iter().filter(|id| id.ends_with("Z")).count();
        if hits > 2 {
            println!(
                "Move #{}: I'm at {}; {} hits",
                steps,
                currents.iter().join(", "),
                hits
            );
        }
        if steps == 10_000_000_000 {
            return Err("oy".to_string());
        }
        if currents.iter().all(|id| id.ends_with("Z")) {
            return Ok(steps);
        }
    }
}
