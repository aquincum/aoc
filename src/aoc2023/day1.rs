use crate::common::day::{Day, Question};
use std::str::{Chars, MatchIndices};
use itertools::Itertools;

pub struct Day1;

impl Day for Day1{
    fn question(&self, input: &str, question: Question) {
        let result = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{}", result);
    }

    fn test_data(&self) -> String {
        return "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".to_string();
    }
}


struct DigitMatches<'a> {
    matches: Vec<usize>,
    spelled: &'a str,
    value: u8,
}

#[derive(Clone, Copy)]
struct Digit {
    value: u8,
    pos: usize
}

fn calibrate_with_spelled(line: &str) -> u32 {
    // line
    //     .replace("one", "1")
    //     .replace("two", "2")
    //     .replace("three", "3")
    //     .replace("four", "4")
    //     .replace("five", "5")
    //     .replace("six", "6")
    //     .replace("seven", "7")
    //     .replace("eight", "8")
    //     .replace("nine", "9")
    let digits = vec![("one", 1u8), ("two", 2u8), ("three", 3u8), ("four", 4u8), ("five", 5u8), ("six", 6u8), ("seven",7u8), ("eight", 8u8), ("nine", 9u8)];
    let matches = digits.iter().map(|(spelled,value)| DigitMatches{
        matches: line.match_indices(spelled).map(|(a,_)| a).collect(),
        spelled,
        value: value.clone(),
    });
    let spelled_digits = matches.
        map(|digitmatches| digitmatches.matches.iter().map(|m| Digit{
            value: digitmatches.value.clone(),
            pos: m.clone(),
        }).collect_vec()).flatten().collect_vec();
    let number_digits = line.chars().enumerate().filter(|(_,c)| c.is_digit(10)).map(|(idx, c)| Digit{
        value: c.to_digit(10).unwrap() as u8,
        pos: idx,
    }).collect_vec();
    let all_digits = [spelled_digits,number_digits].concat();
    let min_digit = all_digits.iter().min_by_key(|d| d.pos).unwrap();
    let max_digit = all_digits.iter().max_by_key(|d| d.pos).unwrap();

    // let min = matches.map(|mi: MatchIndices<_>| mi.min() ).filter(|m| m.is_some()).map(|m| m.unwrap()).min
    (min_digit.value*10+max_digit.value) as u32
}

fn calibration_value(line: &str) -> u32 {
    let chars: Chars = line.chars();
    let digits = chars.filter(|c| c.is_digit(10)).collect_vec();
    let first = digits.get(0).unwrap();
    let last = digits.get(digits.len()-1).unwrap();
    first.to_digit(10).unwrap()*10+last.to_digit(10).unwrap()
}

fn q1(input: &str) -> String{
    let sum: u32 = input.lines().map(calibration_value).sum();
    format!("{}", sum)
}

fn q2(input: &str) -> String {
    let sum: u32 = input.lines().map(calibrate_with_spelled).sum();
    format!("{}", sum)
}