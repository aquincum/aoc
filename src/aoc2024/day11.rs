use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day11;

impl Day for Day11 {
    fn question(&self, input: &str, question: Question) {
        let res = q2(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "125 17".to_string()
    }
}

type Stone = u128;

fn even_digits(stone: Stone) -> Option<(Stone, Stone)> {
    if stone == 0 {
        None // shouldn't get here but let's not panic
    } else {
        let digits = stone.checked_ilog10().unwrap() + 1;
        if digits.clone() % 2 == 0 {
            let mask = 10u128.pow(digits / 2);
            Some(((stone.clone() / mask.clone()), (stone % mask)))
        } else {
            None
        }
    }
}

fn q1(input: &str, question: Question) -> Result<u128, String> {
    let stones: Result<Vec<Stone>, String> = input
        .split(" ")
        .map(|s| s.parse().map_err(|e: ParseIntError| e.to_string()))
        .collect();
    let stones = stones?;
    let target = match question {
        Question::First => 25,
        Question::Second => 75,
    };
    let end_stones = (0usize..target).fold(stones, |stones, i| {
        println!("{} {}", i, stones.len());
        for (k, v) in stones.iter().counts().iter() {
            println!("{}: {}", k, v);
        }
        stones
            .into_iter()
            .map(|stone| {
                if stone == 0 {
                    vec![1]
                } else if let Some((st1, st2)) = even_digits(stone) {
                    vec![st1, st2]
                } else {
                    vec![stone * 2024]
                }
            })
            .flatten()
            .collect()
    });
    Ok(end_stones.len() as u128)
}

struct StoneNode {
    count: u128,
    next1: Option<u128>,
    next2: Option<u128>,
}

fn update_with_count(mut map: &mut HashMap<Stone, StoneNode>, key: Stone, count: u128) {
    map.entry(key)
        .and_modify(|s| s.count += count)
        .or_insert(StoneNode {
            count: count.clone(),
            next1: None,
            next2: None,
        });
}

fn q2(input: &str, question: Question) -> Result<u128, String> {
    let stones: Result<Vec<Stone>, String> = input
        .split(" ")
        .map(|s| s.parse().map_err(|e: ParseIntError| e.to_string()))
        .collect();
    let stones = stones?;
    let target = match question {
        Question::First => 25,
        Question::Second => 75,
    };
    let mut stonemap: HashMap<Stone, StoneNode> = stones
        .into_iter()
        .map(|stone| {
            (
                stone,
                StoneNode {
                    count: 1,
                    next1: None,
                    next2: None,
                },
            )
        })
        .collect();
    let end_stones = (0usize..target).fold(stonemap, |stones, i| {
        let mut newmap = HashMap::new();
        stones.iter().for_each(|(stone, node)| {
            println!("ALRIGHT dealing with {}", stone.clone());
            if let Some(n1) = node.next1.clone() {
                update_with_count(&mut newmap, n1, node.count.clone());
                if let Some(n2) = node.next2.clone() {
                    update_with_count(&mut newmap, n2, node.count.clone());
                }
            } else {
                if stone.clone() == 0 {
                    update_with_count(&mut newmap, 1, node.count.clone());
                    newmap
                        .entry(stone.clone())
                        .and_modify(|sn| sn.next1 = Some(1));
                } else if let Some((st1, st2)) = even_digits(stone.clone()) {
                    update_with_count(&mut newmap, st1, node.count.clone());
                    update_with_count(&mut newmap, st2, node.count.clone());
                    newmap.entry(stone.clone()).and_modify(|sn| {
                        sn.next1 = Some(st1.clone());
                        sn.next2 = Some(st2.clone());
                    });
                } else {
                    update_with_count(&mut newmap, stone.clone() * 2024, node.count.clone());
                    newmap
                        .entry(stone.clone())
                        .and_modify(|sn| sn.next1 = Some(stone.clone() * 1024));
                }
            }
        });
        newmap.iter_mut().for_each(|(stone, node)| {
            node.next1 = stones.get(stone).map(|n| n.next1.clone()).flatten();
            node.next2 = stones.get(stone).map(|n| n.next2.clone()).flatten();
        });
        println!("\n\nAt {}", i);
        for (k, v) in &newmap {
            println!("{}: {}", k, v.count);
        }
        newmap
    });
    Ok(end_stones.values().into_iter().map(|sm| sm.count).sum())
}
