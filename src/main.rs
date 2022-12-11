use std::fs;
mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod hackerrank;

fn must_read_file() -> String {
    fs::read_to_string("input.txt").expect("reading in file")
}

fn main() {
    // hackerrank::main();
    let input = must_read_file();
    // let result_q1 = day6::question(&input, 4);
    // let result_q2 = day6::question(&input, 14);
    // println!("{:?} {:?}", result_q1, result_q2)
    day11::question(&input, 2);
}
