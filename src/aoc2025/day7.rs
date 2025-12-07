use crate::common::columns::Columnser;
use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Day7;

impl Day for Day7 {
    fn question(&self, input: &str, question: Question) {
        let splitters = input
            .lines()
            .map(|l| l.parse::<SplitterRow>().unwrap())
            .collect_vec();
        let start = input
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_, ch)| *ch == 'S')
            .map(|(i, _)| i)
            .next()
            .unwrap();
        // sigh
        let width = input.lines().nth(0).unwrap().len();
        let mut splits = 0usize;
        let mut currents = HashMap::new();
        currents.insert(start, 1u128);
        for row in splitters {
            let mut newcurrents = HashMap::new();
            for (curr, lives) in &currents {
                if row.0.contains(curr) {
                    splits += 1;
                    if *curr > 0 {
                        let entry = newcurrents.entry(curr - 1).or_insert(0u128);
                        *entry += lives;
                    }
                    if *curr < width - 1 {
                        let entry = newcurrents.entry(curr + 1).or_insert(0u128);
                        *entry += lives;
                    }
                } else {
                    let entry = newcurrents.entry(*curr).or_insert(0u128);
                    *entry += lives;
                }
            }
            currents = newcurrents;
        }
        let all_lives = currents.iter().map(|(_, v)| v).sum::<u128>();
        println!("Question: q1 {} q2 {}", splits, all_lives);
    }

    fn test_data(&self) -> String {
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
        .to_string()
    }
}

struct SplitterRow(HashSet<usize>);

impl FromStr for SplitterRow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let set = s
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '^')
            .map(|(i, _)| i)
            .collect();
        Ok(SplitterRow(set))
    }
}
