use std::fs;
mod day1;
mod day2;
mod day3;
mod hackerrank;

fn must_read_file() -> String {
    fs::read_to_string("input.txt").expect("reading in file")
}

fn main() {
    hackerrank::main();
    let input = must_read_file();
    let result = day3::q2(&input);
    println!("{} {}", result, day3::q2_itertools(&input));
}
