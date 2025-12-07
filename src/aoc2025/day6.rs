use crate::common::columns::Columnser;
use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day6;

impl Day for Day6 {
    fn question(&self, input: &str, question: Question) {
        let n = input.lines().count();
        let num_map: Vec<Vec<u128>> = read_num_map(input, n, question);
        println!("{:?}", num_map);
        let ops: Vec<Operation> = input
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .map(|op| op.parse().unwrap())
            .collect();

        let result = ops.iter().enumerate().fold(0u128, |sum, (i, op)| {
            let ns = num_map.iter().map(|row| row[i]);
            let local_res = ns.fold(op.zero(), |acc, x| op.do_operation(acc, x));
            println!("{}: {}", i, local_res);
            sum + local_res
        });
        println!("Question: {}", result);
    }

    fn test_data(&self) -> String {
        "123 328  51 640
 45 64  387 230
  6 98  215 314
*   +   *   +  "
            .to_string()
    }
}

fn read_num_map(input: &str, n: usize, question: Question) -> Vec<Vec<u128>> {
    match question {
        Question::First => input
            .lines()
            .take(n - 1)
            .map(|l| {
                l.split_whitespace()
                    .map(|x| x.parse::<u128>().unwrap())
                    .collect()
            })
            .collect(),
        Question::Second => q2(input, n),
    }
}

fn q2(input: &str, n: usize) -> Vec<Vec<u128>> {
    let op_indices: HashSet<usize> = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, ch)| *ch == '*' || *ch == '+')
        .map(|(i, _)| i)
        .collect();
    let num_arrs = input
        .lines()
        .take(n - 1)
        .map(|l| {
            let (nums, current) =
                l.chars()
                    .enumerate()
                    .fold((vec![], vec![]), |(nums, current), (i, ch)| {
                        let cnum = ch.to_digit(10);
                        if op_indices.contains(&(i + 1)) {
                            (nums, current)
                        } else if op_indices.contains(&i) && i != 0 {
                            (vec![nums, vec![current]].concat(), vec![cnum])
                        } else {
                            (nums, vec![current, vec![cnum]].concat())
                        }
                    });
            println!("AAIIAIA {:?} {:?}", nums, current);
            vec![nums, vec![current]].concat()
        })
        .collect_vec();
    let all_nums = op_indices
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let ns = num_arrs.iter().map(|row| row[i].clone()).collect_vec();
            println!("A {}: {:?}", i, ns);
            let ns = (0..ns[0].len())
                .map(|i| ns.iter().map(|n| n[i]).collect_vec())
                .collect_vec();
            println!("B {}: {:?}", i, ns);
            let ns = ns.iter().map(|digits| {
                println!("My current digs are : {:?}", digits);
                let len = digits.len();
                digits
                    .iter()
                    .filter(|n| n.is_some())
                    .rev()
                    .enumerate()
                    .map(|(i, digit)| (digit.unwrap_or(0) as u128) * 10u128.pow(i as u32))
                    .sum::<u128>()
            });
            ns.collect_vec()
        })
        .collect_vec();
    let ops: Vec<Operation> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|op| op.parse().unwrap())
        .collect();

    let result = ops.iter().enumerate().fold(0u128, |sum, (i, op)| {
        let local_res = all_nums[i]
            .iter()
            .fold(op.zero(), |acc, n| op.do_operation(acc, *n));
        println!("{}: {}", i, local_res);
        sum + local_res
    });
    println!("Question: {}", result);
    vec![]
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Operation {
    Add,
    Mult,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Mult),
            _ => Err(format!("Invalid operation {}", s)),
        }
    }
}

impl Operation {
    fn do_operation(&self, i1: u128, i2: u128) -> u128 {
        match self {
            Operation::Add => i1 + i2,
            Operation::Mult => i1 * i2,
        }
    }
    fn zero(&self) -> u128 {
        match self {
            Operation::Add => 0,
            Operation::Mult => 1,
        }
    }
}
