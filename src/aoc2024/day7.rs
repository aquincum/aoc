use crate::common::day::{Day, Question};
use clap::builder::TypedValueParser;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day7;

impl Day for Day7 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .to_string()
    }
}

struct Equation {
    result: u128,
    nums: Vec<u128>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Add => "+".to_string(),
            Operation::Multiply => "*".to_string(),
            Operation::Concatenate => "||".to_string(),
        }
    }
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, nums) = s
            .split(": ")
            .collect_tuple()
            .ok_or(format!("{}: not correct line", s))?;
        let result = result.parse().map_err(|e: ParseIntError| e.to_string())?;
        let nums: Result<Vec<u128>, String> = nums
            .split(" ")
            .map(|s| s.parse().map_err(|e: ParseIntError| e.to_string()))
            .collect();
        let nums = nums?;
        Ok(Equation { result, nums })
    }
}
impl Equation {
    //noinspection RsTypeCheck
    fn fits(&self, current: u128, from: usize, use_concat: bool) -> Option<Vec<Operation>> {
        if from == self.nums.len() {
            if current == self.result {
                Some(vec![])
            } else {
                None
            }
        } else {
            let plusbranch = current + self.nums[from];
            let plusbranch = if plusbranch > self.result {
                None
            } else {
                if let Some(ops) = self.fits(plusbranch, from + 1, use_concat) {
                    Some(
                        vec![ops, vec![Operation::Add]]
                            .into_iter()
                            .flatten()
                            .collect_vec(),
                    )
                } else {
                    None
                }
            };
            if plusbranch.is_some() {
                return plusbranch; // ouch
            }

            let multbranch = current * self.nums[from];
            let multbranch = if multbranch > self.result {
                None
            } else {
                if let Some(ops) = self.fits(multbranch, from + 1, use_concat) {
                    Some(
                        vec![vec![Operation::Multiply], ops]
                            .into_iter()
                            .flatten()
                            .collect_vec(),
                    )
                } else {
                    None
                }
            };
            if multbranch.is_some() {
                return multbranch; // ouch
            }

            if use_concat {
                let concated: String = format!("{}{}", current, self.nums[from]);
                let concated: u128 = concated.parse::<u128>().unwrap();
                if let Some(ops) = self.fits(concated, from + 1, use_concat) {
                    return Some(
                        vec![vec![Operation::Concatenate], ops]
                            .into_iter()
                            .flatten()
                            .collect_vec(),
                    );
                }
            }
            None
        }
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let eqs: Result<Vec<Equation>, String> = input.lines().map(|l| l.parse()).collect();
    let eqs = eqs?;
    let sum = eqs
        .iter()
        .filter(|e| {
            let ops = e.fits(0, 0, question == Question::Second);
            if ops.is_some() {
                print!("{} = ", e.result);
                let ops = ops.unwrap();
                let mut ops_str = ops.iter().map(|op| op.to_string());
                let intersperse_closure = || ops_str.next().unwrap_or("?".to_string());
                println!(
                    "{}",
                    e.nums
                        .iter()
                        .map(|n| format!("{}", n))
                        .intersperse_with(intersperse_closure)
                        .join(" ")
                );
                true
            } else {
                false
            }
        })
        .fold(0, |acc, e| acc + e.result);
    Ok(sum)
}
