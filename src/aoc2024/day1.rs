use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::num::ParseIntError;

pub struct Day1;

impl Day for Day1 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string()
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    // let lists = input.lines().map(|l| l.split("   ").map(|s| s.parse().map_err(|e: ParseIntError| e.to_string())));
    let pairs: Vec<(&str, &str)> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect_tuple())
        .filter_map(|x| x)
        .collect_vec();
    let list0 = pairs.iter().map(|x| x.0).collect_vec();
    let list0: Result<Vec<i128>, String> = list0
        .into_iter()
        .map(|x| x.parse().map_err(|e: ParseIntError| e.to_string()))
        .collect();
    let list0 = list0?;
    let list0: Vec<i128> = list0.into_iter().sorted().collect();

    let list1 = pairs.iter().map(|x| x.1).collect_vec();
    let list1: Result<Vec<i128>, String> = list1
        .into_iter()
        .map(|x| x.parse().map_err(|e: ParseIntError| e.to_string()))
        .collect();
    let list1 = list1?;
    let list1: Vec<i128> = list1.into_iter().sorted().collect();

    if question == Question::First {
        Ok((0..pairs.len())
            .map(|i| (list0.get(i).unwrap() - list1.get(i).unwrap()).abs())
            .map(|x| {
                println!("DIF  {}", x);
                x
            })
            .sum::<i128>() as u128)
    } else {
        let counter1 = list1.into_iter().counts();
        Ok(list0
            .iter()
            .map(|n| counter1.get(n).unwrap_or(&0).clone() as u128 * *n as u128)
            .sum::<u128>())
    }
}
