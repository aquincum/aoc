use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day9;

impl Day for Day9 {
    fn question(&self, input: &str, question: Question) {
        match q(input, question) {
            Ok(n) => println!("{}", n),
            Err(err) => println!("{}", err),
        }
    }

    fn test_data(&self) -> String {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .to_string()
    }
}

struct Series {
    numbers: Vec<i128>,
}

impl FromStr for Series {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Result<_, _> = s.split_ascii_whitespace().map(|s| s.parse()).collect();
        let numbers = numbers.map_err(|e: ParseIntError| e.to_string())?;
        Ok(Series { numbers })
    }
}

impl Series {
    fn derive(&self) -> Series {
        // let mut it = self.numbers.iter();
        // let mut prev = it.next();
        let diffs = self.numbers.iter().fold::<(Vec<_>, Option<&i128>), _>(
            (vec![], None),
            |(mut built, last), current| {
                if let Some(last) = last {
                    built.push(current - last);
                }
                (built, Some(current))
            },
        );
        Series { numbers: diffs.0 }
    }
    fn is_zero(&self) -> bool {
        self.numbers.iter().all(|x| x == &0)
    }
    fn next_num(&self, question: Question) -> i128 {
        if self.is_zero() {
            0
        } else {
            let derived = self.derive();
            let derived_next = derived.next_num(question);
            println!(
                "Deriving for {}: {}",
                self.numbers.iter().map(|n| n.to_string()).join(","),
                derived_next
            );
            match question {
                Question::First => self.numbers.last().unwrap() + derived_next,
                Question::Second => self.numbers.first().unwrap() - derived_next,
            }
        }
    }
}

fn q(input: &str, question: Question) -> Result<i128, String> {
    let series_plural: Result<Vec<Series>, _> = input.lines().map(|l| l.parse()).collect();
    let series_plural = series_plural?;
    let nexts = series_plural
        .iter()
        .map(|series| series.next_num(question))
        .sum();
    Ok(nexts)
}
