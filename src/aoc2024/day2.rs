// baaaaaaaaaaaaaaad traveling day

use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day2;

impl Day for Day2 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string()
    }
}

struct Report {
    nums: Vec<u128>,
}

impl FromStr for Report {
    type Err = String;

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let nums: Result<Vec<u128>, String> = l
            .split_ascii_whitespace()
            .map(|s| s.parse::<u128>().map_err(|e: ParseIntError| e.to_string()))
            .collect();
        let nums = nums?;
        Ok(Report { nums: nums })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Start,
    FirstElement,
    Increase,
    Decrease,
}

impl Report {
    fn pass(&self) -> bool {
        let (good, _, _) =
            self.nums
                .iter()
                .fold((true, 0, Direction::Start), |(good, prev, dir), &n| {
                    // println!("{} {} {:?}", good, prev, dir);
                    if !good {
                        (good, n, dir)
                    } else if dir == Direction::Start {
                        (true, n, Direction::FirstElement)
                    } else if prev == n {
                        (false, n, dir)
                    } else if (n as i128).abs_diff(prev as i128) > 3 {
                        (false, n, dir)
                    } else if dir == Direction::FirstElement {
                        (
                            true,
                            n,
                            if prev < n {
                                Direction::Increase
                            } else {
                                Direction::Decrease
                            },
                        )
                    } else if dir == Direction::Decrease && prev < n {
                        (false, n, dir)
                    } else if dir == Direction::Increase && prev > n {
                        (false, n, dir)
                    } else {
                        println!("OKAY {} {} {:?}", prev, n, dir);
                        (true, n, dir)
                    }
                });
        println!("{}", good);
        good
    }
    fn permutate(&self) -> impl Iterator<Item = Report> + '_ {
        (0..self.nums.iter().len()).map(move |skip| Report {
            nums: self
                .nums
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip)
                .map(|(_, x)| *x)
                .collect_vec(),
        })
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let reports: Result<Vec<Report>, String> = input.lines().map(|l| l.parse()).collect();
    let reports = reports?;
    println!("{}", reports.len());
    if question == Question::First {
        Ok(reports.into_iter().filter(|r| r.pass()).count() as u128)
    } else {
        Ok(reports
            .into_iter()
            .map(|r| {
                r.permutate().any(|r2| {
                    println!("PERMI {:?} {}", r2.nums, r2.pass());
                    r2.pass()
                })
            })
            .filter(|b| *b)
            .count() as u128)
    }
}
