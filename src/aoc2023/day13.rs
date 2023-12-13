use crate::common::columns::Columnser;
use crate::common::day::{Day, Question};
use bit_vec::BitVec;
use itertools::Itertools;
use std::iter::FromIterator;
use std::str::FromStr;

pub struct Day13;

impl Day for Day13 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res)
    }

    fn test_data(&self) -> String {
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"
        .to_string()
    }
}

struct Map {
    rows: Vec<BitVec>,
    columns: Vec<BitVec>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|l| {
                let chars = l.chars().map(|ch| ch == '#');
                BitVec::<u32>::from_iter(chars)
            })
            .collect_vec();
        let columns = s
            .columns(s.lines().nth(0).ok_or("Empty first line")?.len())
            .map(|col| {
                let chars = col.chars().map(|ch| ch == '#');
                BitVec::<u32>::from_iter(chars)
            })
            .collect_vec();
        Ok(Map { rows, columns })
    }
}

impl Map {
    fn row_mirror(&self) -> Option<usize> {
        (1..self.rows.len())
            .position(|idx| is_mirror(&self.rows, idx))
            .map(|n| n + 1)
    }
    fn col_mirror(&self) -> Option<usize> {
        (1..self.columns.len())
            .position(|idx| is_mirror(&self.columns, idx))
            .map(|n| n + 1)
    }
    fn row_mirror_smudge(&self) -> Option<usize> {
        (1..self.rows.len())
            .position(|idx| mirror_diffs(&self.rows, idx) == 1)
            .map(|n| n + 1)
    }
    fn col_mirror_smudge(&self) -> Option<usize> {
        (1..self.columns.len())
            .position(|idx| mirror_diffs(&self.columns, idx) == 1)
            .map(|n| n + 1)
    }
}

fn is_mirror(vs: &Vec<BitVec>, breakpoint: usize) -> bool {
    let mut left = breakpoint - 1;
    let mut right = breakpoint;
    loop {
        if vs.get(left).unwrap() != vs.get(right).unwrap() {
            return false;
        }
        if left == 0 || right == vs.len() - 1 {
            return true;
        }
        left -= 1;
        right += 1;
    }
    panic!("shouldn't get here")
}

fn mirror_diffs(vs: &Vec<BitVec>, breakpoint: usize) -> usize {
    let mut left = breakpoint - 1;
    let mut right = breakpoint;
    let mut diffs = 0;
    loop {
        let mut lclone = vs.get(left).unwrap().clone();
        let r = vs.get(right).unwrap();
        lclone.xor(r);
        diffs += lclone.iter().filter(|x| *x).count();
        if diffs > 1 {
            return diffs; // short circuit
        }
        if left == 0 || right == vs.len() - 1 {
            return diffs;
        }
        left -= 1;
        right += 1;
    }
    panic!("shouldn't get here")
}

fn q(input: &str, question: Question) -> Result<usize, String> {
    let maps: Result<Vec<Map>, String> = input.split("\n\n").map(|s| s.parse()).collect();
    let maps = maps?;
    let sum: Result<Vec<usize>, String> = maps
        .iter()
        .map(|m| {
            let row_mirror = match question {
                Question::First => m.row_mirror(),
                Question::Second => m.row_mirror_smudge(),
            };
            let col_mirror = match question {
                Question::First => m.col_mirror(),
                Question::Second => m.col_mirror_smudge(),
            };
            if let Some(row_mirror) = row_mirror {
                Ok(row_mirror * 100)
            } else if let Some(col_mirror) = col_mirror {
                Ok(col_mirror)
            } else {
                Err("no col or row mirror".to_string())
            }
        })
        .collect();
    let sum = sum?.iter().sum();
    Ok(sum)
}
