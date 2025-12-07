use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day22;

impl Day for Day22 {
    fn question(&self, input: &str, question: Question) {
        println!("{:?}", q(input, question));
    }

    fn test_data(&self) -> String {
        "1
10
100
2024"
            .to_string()
    }
}

fn mix(a: u128, b: u128) -> u128 {
    a ^ b
}

fn prune(a: u128) -> u128 {
    a % 16777216
}

fn next_random(x: u128) -> u128 {
    let a = x * 64;
    let x_1 = prune(mix(x, a));
    let b = x_1 / 32;
    let x_2 = prune(mix(x_1, b));
    let c = x_2 * 2048;
    prune(mix(x_2, c))
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let starts: Result<Vec<u128>, _> = input
        .lines()
        .map(|s| s.parse::<u128>().map_err(|e| e.to_string()))
        .collect();
    let starts = starts?;
    if question == Question::First {
        let ends = starts
            .iter()
            .map(|start| (0..2000).fold(*start, |secret, i| next_random(secret)))
            .collect_vec();
        Ok(ends.into_iter().sum())
    } else {
        let mut winnings_map = HashMap::new();
        for start in starts {
            let mut seen_map = HashSet::new();
            let mut queue = VecDeque::new();
            let mut old_secret = start;
            // println!("{}: {}", old_secret, old_secret % 10);
            for i in 1..2000 {
                let new_secret = next_random(old_secret);
                let winnings = new_secret % 10;
                let delta = winnings as i32 - (old_secret % 10) as i32;
                // println!("{}: {} ({})", new_secret, winnings, delta);
                queue.push_front(delta);
                if queue.len() == 5 {
                    queue.pop_back();
                }
                if queue.len() == 4 {
                    let tup: (i32, i32, i32, i32) =
                        queue.iter().map(|x| *x).collect_tuple().unwrap();
                    if !seen_map.contains(&tup) {
                        seen_map.insert(tup.clone());
                        winnings_map
                            .entry(tup)
                            .and_modify(|x| *x += winnings.clone())
                            .or_insert(winnings);
                        // println!("Inserted {:?} with {}", tup, winnings);
                    }
                }
                old_secret = new_secret;
            }
            // println!("\n\n\n=========================================\n=========================================\n=========================================\n\n\n\n")
        }
        winnings_map
            .iter()
            .max_by_key(|(_, v)| **v)
            .map(|n| {
                println!("{:?}", n);
                *n.1
            })
            .ok_or("no max?".to_string())
    }
}
