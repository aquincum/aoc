use crate::common::day::Day;
use clap::Parser;
use common::day::Question;
use itertools::Itertools;
use phf::phf_map;
use serde_json::ser::CharEscape::Quote;
use std::any::Any;
use std::fs;

mod aoc2022;
mod aoc2023;
mod aoc2024;
mod common;
mod hackerrank;

#[derive(Parser)]
#[command(name = "aoc", author, version, about, long_about = None)]
struct Cli {
    #[arg(help = "[defaults to the last day]")]
    day: Option<u8>,
    #[arg(help = "[defaults to the last year]")]
    year: Option<u16>,
    #[arg(short, value_parser=clap::value_parser!(u8).range(1..3), default_value_t=1)]
    question: u8,
    #[arg(short, long, default_value = "input.txt")]
    file_name: String,
    #[arg(short, long)]
    test: bool,

    #[arg(short, long)]
    list_days: bool,
}

fn must_read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("reading in file")
}

const DAYS_2024: phf::Map<u8, &'static dyn Day> = phf_map! {
    1u8 => &aoc2024::day1::Day1,
    2u8 => &aoc2024::day2::Day2,
    3u8 => &aoc2024::day3::Day3,
    4u8 => &aoc2024::day4::Day4,
    5u8 => &aoc2024::day5::Day5,
    6u8 => &aoc2024::day6::Day6,
    7u8 => &aoc2024::day7::Day7,
};

const DAYS_2023: phf::Map<u8, &'static dyn Day> = phf_map! {
    1u8 => &aoc2023::day1::Day1,
    2u8 => &aoc2023::day2::Day2,
    3u8 => &aoc2023::day3::Day3,
    4u8 => &aoc2023::day4::Day4,
    5u8 => &aoc2023::day5::Day5,
    6u8 => &aoc2023::day6::Day6,
    7u8 => &aoc2023::day7::Day7,
    8u8 => &aoc2023::day8::Day8,
    9u8 => &aoc2023::day9::Day9,
    10u8 => &aoc2023::day10::Day10,
    11u8 => &aoc2023::day11::Day11,
    12u8 => &aoc2023::day12::Day12,
    13u8 => &aoc2023::day13::Day13,
    14u8 => &aoc2023::day14::Day14,
    15u8 => &aoc2023::day15::Day15,
    16u8 => &aoc2023::day16::Day16,
    17u8 => &aoc2023::day17::Day17,
    18u8 => &aoc2023::day18::Day18,
    19u8 => &aoc2023::day19::Day19,
    20u8 => &aoc2023::day20::Day20,
    21u8 => &aoc2023::day21::Day21,
    22u8 => &aoc2023::day22::Day22,
    // 23u8 => &aoc2023::day23::Day23,
    // 24u8 => &aoc2023::day24::Day24,
    // 25u8 => &aoc2023::day25::Day25,
};

const DAYS_2022: phf::Map<u8, &'static dyn Day> = phf_map! {
    1u8 => &aoc2022::day1::Day1,
    2u8 => &aoc2022::day2::Day2,
    3u8 => &aoc2022::day3::Day3,
    4u8 => &aoc2022::day4::Day4,
    5u8 => &aoc2022::day5::Day5,
    6u8 => &aoc2022::day6::Day6,
    7u8 => &aoc2022::day7::Day7,
    8u8 => &aoc2022::day8::Day8,
    9u8 => &aoc2022::day9::Day9,
    10u8 => &aoc2022::day10::Day10,
    11u8 => &aoc2022::day11::Day11,
    12u8 => &aoc2022::day12::Day12,
    13u8 => &aoc2022::day13::Day13,
    14u8 => &aoc2022::day14::Day14,
    15u8 => &aoc2022::day15::Day15,
    16u8 => &aoc2022::day16::Day16,
    17u8 => &aoc2022::day17::Day17,
    18u8 => &aoc2022::day18::Day18,
    19u8 => &aoc2022::day19::Day19,
    20u8 => &aoc2022::day20::Day20,
    21u8 => &aoc2022::day21::Solution,
};

const YEARS: phf::Map<u16, phf::Map<u8, &'static dyn Day>> = phf_map! {
    2022u16 => DAYS_2022,
    2023u16 => DAYS_2023,
    2024u16 => DAYS_2024,
};

fn print_available_days() {
    println!(
        "Available days:\n{}",
        YEARS
            .entries()
            .map(|(yr, days)| format!("YEAR {}: {}", yr, days.keys().sorted().join(", ")))
            .join("\n")
    );
}

fn main() {
    let cli: Cli = Cli::parse();

    if cli.list_days {
        print_available_days();
        return;
    }

    let question = match cli.question {
        1 => Question::First,
        2 => Question::Second,
        _ => panic!("question"),
    };
    let year_n = match cli.year {
        Some(y) if y < 2000 => y + 2000,
        Some(y) => y,
        None => YEARS.keys().max().unwrap().clone(),
    };
    let days = YEARS.get(&year_n);
    if days.is_none() {
        println!(
            "Non existent year! Available years: {}",
            YEARS.keys().sorted().join(", ")
        );
        return;
    }
    let days = days.unwrap();
    let day_n = match cli.day {
        Some(d) => d,
        None => days.keys().max().unwrap().clone(),
    };
    let day = days.get(&day_n);
    if day.is_none() {
        print!("Non existent day!");
        print_available_days();
        return;
    }
    let day = day.unwrap();
    let input = if cli.test {
        day.test_data()
    } else {
        must_read_file(&cli.file_name)
    };
    println!(
        "Running year {} day {}, {:?} question",
        year_n, day_n, question
    );
    day.question(&input, question);

    // hackerrank::main();
    // let input = must_read_file();
    // let result_q1 = day6::question(&input, 4);
    // let result_q2 = day6::question(&input, 14);
    // println!("{:?} {:?}", result_q1, result_q2)
    // day14::question(&input, Question::Second);
    // day20::question(&input, Question::Second);
}
