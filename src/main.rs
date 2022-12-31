use std::fs;
use serde_json::ser::CharEscape::Quote;
use common::day::Question;
use clap::Parser;
use crate::common::day::Day;
use phf::phf_map;
use itertools::Itertools;
use std::any::Any;

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
mod day18;
mod day19;
mod day20;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod hackerrank;

#[derive(Parser)]
#[command(name = "aoc", author, version, about, long_about = None)]
struct Cli {
    #[arg(help="[defaults to the last day]")]
    day: Option<u8>,
    #[arg(value_parser=clap::value_parser!(u8).range(1..3), default_value_t=1)]
    question: u8,
    #[arg(short,long,default_value="input.txt")]
    file_name: String,
    #[arg(short, long)]
    test: bool,

    #[arg(short, long)]
    list_days: bool,
}


fn must_read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("reading in file")
}

const DAYS: phf::Map<u8, &'static dyn Day> = phf_map! {
    1u8 => &day1::Day1,
    2u8 => &day2::Day2,
    3u8 => &day3::Day3,
    4u8 => &day4::Day4,
    5u8 => &day5::Day5,
    6u8 => &day6::Day6,
    7u8 => &day7::Day7,
    8u8 => &day8::Day8,
    9u8 => &day9::Day9,
    10u8 => &day10::Day10,
    11u8 => &day11::Day11,
    12u8 => &day12::Day12,
    13u8 => &day13::Day13,
    14u8 => &day14::Day14,
    15u8 => &day15::Day15,
    16u8 => &day16::Day16,
    17u8 => &day17::Day17,
    18u8 => &day18::Day18,
    19u8 => &day19::Day19,
    20u8 => &day20::Day20,
};

fn main() {
    let cli: Cli = Cli::parse();

    if cli.list_days {
        println!("Available days: {}", DAYS.keys().sorted().join(", "));
        return;
    }

    let question = match cli.question {
        1 => Question::First,
        2 => Question::Second,
        _ => panic!("question")
    };
    let day_n = match cli.day {
        Some(d) => d,
        None => DAYS.keys().max().unwrap().clone()
    };
    let day = DAYS.get(&day_n);
    if day.is_none() {
        println!("Non existent day! Avaliable days: {}", DAYS.keys().sorted().join(", "));
        return;
    }
    let day = day.unwrap();
    let input = if cli.test {
        day.test_data()
    } else {
        must_read_file(&cli.file_name)
    };
    println!("Running day {}, {:?} question", day_n, question);
    day.question(&input, question);

    // hackerrank::main();
    // let input = must_read_file();
    // let result_q1 = day6::question(&input, 4);
    // let result_q2 = day6::question(&input, 14);
    // println!("{:?} {:?}", result_q1, result_q2)
    // day14::question(&input, Question::Second);
    // day20::question(&input, Question::Second);
}
