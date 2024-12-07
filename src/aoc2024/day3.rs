use crate::common::day::{Day, Question};
use regex::Regex;
use std::str::Split;

pub struct Day3;

impl Day for Day3 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let regexp = match question {
        Question::First => Regex::new(r"mul\([0-9]+,[0-9]+\)").map_err(|e| e.to_string())?,
        Question::Second => {
            Regex::new(r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))").map_err(|e| e.to_string())?
        }
    };
    let inner_regexp = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").map_err(|e| e.to_string())?;
    if question == Question::First {
        let pairs = regexp.find_iter(input).map(|m| {
            let caps = inner_regexp.captures(m.as_str()).unwrap();
            let n1: u128 = caps.get(1).unwrap().as_str().parse().unwrap();
            let n2: u128 = caps.get(2).unwrap().as_str().parse().unwrap();
            (n1, n2)
        });
        let sum = pairs.map(|(x, y)| x * y).sum();
        Ok(sum)
    } else {
        let (sum, _) = regexp
            .find_iter(input)
            .fold((0u128, true), |(sumsofar, switch), m| {
                println!("{} {}", m.as_str(), switch);
                if m.as_str() == "do()" {
                    (sumsofar, true)
                } else if m.as_str() == "don't()" {
                    (sumsofar, false)
                } else if !switch {
                    (sumsofar, switch)
                } else {
                    let caps = inner_regexp.captures(m.as_str()).unwrap();
                    let n1: u128 = caps.get(1).unwrap().as_str().parse().unwrap();
                    let n2: u128 = caps.get(2).unwrap().as_str().parse().unwrap();
                    (sumsofar + (n1 * n2), switch)
                }
            });
        Ok(sum)
    }
}
