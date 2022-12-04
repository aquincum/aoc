use std::io;

pub fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input)?;
    let n = input.parse::<i32>().unwrap();
    let range = 0..n;
    // let mut inp_v = Vec::new();
    // for i in 0..n {
    //     stdin.read_line(&mut input);
    //     inp_v.push(input.clone());
    // }
    let nums = range.map(|_| {
        stdin.read_line(&mut input);
        input.clone()
    });
    let sum = nums.fold("0", |sofar, new_n| {
        let digits = sofar.chars().rev().zip(new_n.chars().rev());
        "2"
    });
    Ok(())
}
