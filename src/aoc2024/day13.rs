// originally in the office in python but why not rewrite it

use crate::common::day::{Day, Question};
use regex::Regex;
use std::any::Any;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day13;

impl Day for Day13 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            .to_string()
    }
}

struct Machine {
    ax: u128,
    ay: u128,
    bx: u128,
    by: u128,
    px: u128,
    py: u128,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .map_err(|e| e.to_string())?;
        let caps = re.captures(s).ok_or("no captures".to_string())?;
        let ax = caps
            .get(1)
            .ok_or("no ax")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let ay = caps
            .get(2)
            .ok_or("no ay")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let bx = caps
            .get(3)
            .ok_or("no bx")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let by = caps
            .get(4)
            .ok_or("no by")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let px = caps
            .get(5)
            .ok_or("no px")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let py = caps
            .get(6)
            .ok_or("no py")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;

        Ok(Machine {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        })
    }
}

impl Machine {
    fn to_question_2(self) -> Machine {
        Machine {
            ax: self.ax,
            ay: self.ay,
            bx: self.bx,
            by: self.by,
            px: self.px + 10000000000000,
            py: self.py + 10000000000000,
        }
    }
    fn solution(&self) -> u128 {
        let btop = ((self.ax * self.py) as i128 - (self.ay * self.px) as i128);
        let bbottom = ((self.by * self.ax) as i128 - (self.bx * self.ay) as i128);
        if btop % bbottom == 0 {
            let b = btop / bbottom;
            let atop = (self.px as i128 - (b * self.bx as i128) as i128);
            if atop % self.ax as i128 == 0 {
                let a = atop / self.ax as i128;
                (3 * a + b) as u128
            } else {
                0
            }
        } else {
            0
        }
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let machines: Result<Vec<Machine>, String> = input.split("\n\n").map(|s| s.parse()).collect();
    let machines = machines?;
    let machines: Vec<Machine> = machines
        .into_iter()
        .map(|m| {
            if question == Question::First {
                m
            } else {
                m.to_question_2()
            }
        })
        .collect();
    Ok(machines.iter().map(|m| m.solution()).sum())
}
