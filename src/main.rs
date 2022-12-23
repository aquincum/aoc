use serde_json::ser::CharEscape::Quote;
use std::fs;

mod common;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod hackerrank;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Question {
    First,
    Second,
}

fn must_read_file() -> String {
    fs::read_to_string("input.txt").expect("reading in file")
}

fn main() {
    // hackerrank::main();
    let input = must_read_file();
    // let result_q1 = day6::question(&input, 4);
    // let result_q2 = day6::question(&input, 14);
    // println!("{:?} {:?}", result_q1, result_q2)
    // day14::question(&input, Question::Second);
    day17::question(&input);
}
