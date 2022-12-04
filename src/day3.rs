use itertools::{Chunk, Itertools};
use std::collections::HashMap;

fn lettermap(s: &str) -> HashMap<char, bool> {
    s.chars().map(|c| (c, true)).collect()
}

fn sumlettermap<T>(maps: T) -> HashMap<char, u8>
where
    T: Iterator<Item = HashMap<char, bool>>,
{
    maps.fold(HashMap::new(), |acc, item| {
        item.keys()
            .map(|k| (k.clone(), acc.get(k).map(|x| x.clone()).unwrap_or(0) + 1))
            .collect()
    })
}

fn char_value(c: char) -> u8 {
    if c.is_lowercase() {
        c as u8 - 'a' as u8 + 1
    } else {
        c as u8 - 'A' as u8 + 27
    }
}

pub fn q1(input: &str) -> i32 {
    let rucksack_letters = input
        .lines()
        .map(|l| {
            let compartments = l.split_at(l.len() / 2);
            let (lm1, lm2) = (lettermap(compartments.0), lettermap(compartments.1));
            let shared = lm1
                .keys()
                .filter(|k| lm2.contains_key(k))
                .collect::<Vec<_>>();
            if shared.len() != 1 {
                panic!(
                    "Assumptions not held: {} has less or more than 1 shared char: {}",
                    l,
                    shared.len()
                );
            }
            shared[0].clone()
        })
        .collect::<Vec<_>>();
    rucksack_letters
        .into_iter()
        .map(char_value)
        .map(|n| n as i32)
        .sum()
}

pub fn q2(input: &str) -> i32 {
    let zeros = input
        .lines()
        .enumerate()
        .filter(|(n, _)| n % 3 == 0)
        .map(|(_, x)| x);
    let ones = input
        .lines()
        .enumerate()
        .filter(|(n, _)| n % 3 == 1)
        .map(|(_, x)| x);
    let twos = input
        .lines()
        .enumerate()
        .filter(|(n, _)| n % 3 == 2)
        .map(|(_, x)| x);
    let groups = zeros.zip(ones).zip(twos).map(|((x, y), z)| [x, y, z]);
    groups
        .map(|gr| {
            let lettermaps = gr.iter().map(|s| lettermap(s.clone()));
            let slm = sumlettermap(lettermaps);
            slm
        })
        .map(|summap| summap.iter().find(|(_, v)| **v == 3).unwrap().0.clone())
        .map(|c| char_value(c) as i32)
        .sum()
}
pub fn q2_itertools(input: &str) -> i32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk: itertools::Chunk<_>| sumlettermap(chunk.map(|s| lettermap(s))))
        .map(|summap| summap.iter().find(|(_, v)| **v == 3).unwrap().0.clone())
        .map(|c| char_value(c) as i32)
        .sum()
}
