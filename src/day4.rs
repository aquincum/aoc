use std::borrow::Borrow;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::common::day::{Day, Question};


pub struct Day4;

impl Day for Day4 {
    fn question(&self, input: &str, question: Question) {
        let result = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{}",result);
    }

    fn test_data(&self) -> String {
        return "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".to_string()
    }
}


#[derive(Copy, Clone)]
struct Elf {
    from: i32,
    to: i32,
}

impl FromStr for Elf {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ends: Vec<_> = s.split("-").collect();
        if ends.len() != 2 {
            return Err(format!("Elf {} doesn't have two ends", s));
        }
        let from = ends[0]
            .parse::<i32>()
            .map_err(|e| format!("first elf: {}: {}", ends[0], e))?;
        let to = ends[1]
            .parse::<i32>()
            .map_err(|e| format!("second elf: {}: {}", ends[1], e))?;
        Ok(Elf { from, to })
    }
}

struct ElfPair(Elf, Elf);

impl FromStr for ElfPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves: Result<Vec<_>, Self::Err> = s.split(",").map(|x| x.parse::<Elf>()).collect();
        let elves = elves?;
        if elves.len() != 2 {
            return Err(format!("{} is not an elfpair", s));
        }
        Ok(ElfPair(elves[0], elves[1]))
    }
}

impl ElfPair {
    fn overlap(&self) -> bool {
        let first_bigger = self.0.from <= self.1.from && self.0.to >= self.1.to;
        let second_bigger = self.0.from >= self.1.from && self.0.to <= self.1.to;
        first_bigger || second_bigger
    }

    fn overlap_at_all(&self) -> bool {
        self.0.to >= self.1.from && self.0.from <= self.1.to
    }
}

pub fn q1(input: &str) -> usize {
    let elfpairs = input.lines().map(|s| s.parse::<ElfPair>().unwrap());
    elfpairs.filter(|ep| ep.overlap()).count()
}

pub fn q2(input: &str) -> usize {
    let elfpairs = input.lines().map(|s| s.parse::<ElfPair>().unwrap());
    elfpairs.filter(|ep| ep.overlap_at_all()).count()
}
