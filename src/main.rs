use std::fs;
mod day1;
mod day2;
mod day3;
mod day4;
mod hackerrank;

fn must_read_file() -> String {
    fs::read_to_string("input.txt").expect("reading in file")
}

fn main() {
    // hackerrank::main();
    let input = must_read_file();
    let result1 = day4::q1(&input);
    let result2 = day4::q2(&input);
    println!("{}", result1);
    println!("{}", result2);
}
