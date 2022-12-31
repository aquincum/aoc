use std::num::ParseIntError;
use std::str::FromStr;

use crate::common::day::{Day, Question};

pub struct Day10;

impl Day for Day10 {
    fn question(&self, input: &str, question: Question) {
        crate::day10::question(input);
    }

    fn test_data(&self) -> String {
        return "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".to_string()
    }
}


enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Copy, Clone)]
struct CycleState {
    reg_x: i32,
}

impl CycleState {
    fn signal_strength(&self, cycle_n: i32) -> i32 {
        cycle_n * self.reg_x
    }
    fn draw_pixel(&self, cycle_n: i32) -> bool {
        let diff = (cycle_n - 1) % 40 - self.reg_x;
        diff.abs() < 2
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                let val = parts[1].parse().map_err(|e: ParseIntError| e.to_string())?;
                Ok(Instruction::Addx(val))
            }
            _ => Err(format!("{}: no such instruction", parts[0])),
        }
    }
}

impl Instruction {
    fn run(&self, previous_state: &CycleState) -> Vec<CycleState> {
        match self {
            Instruction::Noop => vec![previous_state.clone()],
            Instruction::Addx(chg) => vec![
                previous_state.clone(),
                CycleState {
                    reg_x: previous_state.reg_x + chg,
                },
            ],
        }
    }
}

pub fn question(input: &str) {
    let instructions = input.lines().map(|l| l.parse::<Instruction>().unwrap());
    let states = instructions.fold(vec![CycleState { reg_x: 1 }], |acc, i| {
        let new_states = i.run(&acc[acc.len() - 1]);
        vec![acc, new_states].concat()
    });
    for (i, s) in states.iter().enumerate() {
        println!(
            "{}: x={} signal_strength={}",
            i + 1,
            s.reg_x,
            s.signal_strength((i + 1) as i32)
        );
    }
    let signal_strengths = states
        .iter()
        .enumerate()
        .map(|(i, st)| st.signal_strength((i + 1) as i32))
        .collect::<Vec<_>>();
    let answer = signal_strengths[19]
        + signal_strengths[59]
        + signal_strengths[99]
        + signal_strengths[139]
        + signal_strengths[179]
        + signal_strengths[219];
    println!("{}", answer);

    for (i, state) in states.iter().enumerate() {
        let pixel = state.draw_pixel((i + 1) as i32);
        let to_draw = if pixel { "#" } else { "." };
        print!("{}", to_draw);
        if i % 40 == 39 {
            println!();
        }
    }
}
