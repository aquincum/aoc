// I've done this with pen n paper and calculated in python at work but let's redo it

use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::num::ParseIntError;

pub struct Day6;

impl Day for Day6 {
    fn question(&self, input: &str, question: Question) {
        match q(input, question) {
            Ok(n) => println!("{}", n),
            Err(err) => println!("{}", err),
        }
    }

    fn test_data(&self) -> String {
        "Time:      7  15   30
Distance:  9  40  200"
            .to_string()
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let races = read_races(input, question)?;
    println!("{:?}", races);
    Ok(races.iter().map(|r| r.race_solutions()).product())
}

#[derive(Debug)]
struct Race {
    time: u128,
    distance: u128,
}

impl Race {
    fn solve_equation(&self) -> (f64, f64) {
        let discriminant = self.time * self.time - 4 * self.distance;
        let discriminant = discriminant as f64;
        (
            (self.time as f64 + discriminant.sqrt()) / 2.0,
            (self.time as f64 - discriminant.sqrt()) / 2.0,
        )
    }
    fn race_solutions(&self) -> u128 {
        let (max, min) = self.solve_equation();
        let min = if min == min.floor() { min + 0.1 } else { min };
        let max = if max == max.ceil() { max - 0.1 } else { max };
        (max.ceil() - min.floor() - 1.0) as u128
    }
}

fn read_races(input: &str, question: Question) -> Result<Vec<Race>, String> {
    let (time_line, dist_line) = input.lines().collect_tuple().ok_or("More lines")?;
    let time_parts = time_line.split_ascii_whitespace().skip(1);
    let dist_parts = dist_line.split_ascii_whitespace().skip(1);
    match question {
        Question::First => {
            let td: Result<Vec<(u128, u128)>, String> = time_parts
                .zip(dist_parts)
                .map(|(t, d)| {
                    let t: Result<u128, String> =
                        t.parse().map_err(|e: ParseIntError| e.to_string());
                    let d: Result<u128, String> =
                        d.parse().map_err(|e: ParseIntError| e.to_string());
                    match (t, d) {
                        (Err(x), Err(y)) => Err(x + &y),
                        (Err(x), Ok(_)) => Err(x),
                        (Ok(_), Err(x)) => Err(x),
                        (Ok(x), Ok(y)) => Ok((x, y)),
                    }
                })
                .collect();
            let td = td?;
            Ok(td
                .into_iter()
                .map(|(time, distance)| Race { time, distance })
                .collect_vec())
        }
        Question::Second => {
            let time = time_parts
                .into_iter()
                .join("")
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?;
            let distance = dist_parts
                .into_iter()
                .join("")
                .parse()
                .map_err(|e: ParseIntError| e.to_string())?;
            Ok(vec![Race { time, distance }])
        }
    }
}
