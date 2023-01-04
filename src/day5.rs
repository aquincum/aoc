use itertools::Itertools;
use std::str::FromStr;

use crate::common::day::{Day, Question};

pub struct Day5;

impl Day for Day5 {
    fn question(&self, input: &str, question: Question) {
        let result = run_question(input, question);
        println!("{}", result);
    }

    fn test_data(&self) -> String {
        return "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();
    }
}

struct CrateLine(Vec<String>);

fn lines_to_stacks(lines: Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    let crates_n = lines.iter().map(|l| l.len()).max().unwrap();
    let mut lls = vec![Vec::new(); crates_n];
    lines.into_iter().rev().for_each(|l| {
        l.into_iter().enumerate().for_each(|(i, bx)| {
            if let Some(bx) = bx {
                lls[i].push(bx);
            }
        })
    });
    lls
}

fn parse_cratelines(lines: Vec<&str>) -> Vec<Vec<Option<char>>> {
    lines
        .into_iter()
        .map(|line| {
            let mut vec = Vec::new();
            for (i, ch) in line.chars().enumerate() {
                // goodbye functional :'(
                if i % 4 == 1 {
                    vec.push(match ch {
                        x if x.is_uppercase() => Some(x),
                        ' ' => None,
                        _ => panic!("What {}", ch),
                    });
                }
            }
            vec
        })
        .collect()
}

#[derive(Debug)]
struct Move {
    how_many: i32,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(" ");
        words.next();
        let how_many = words.next().unwrap().parse().unwrap();
        words.next();
        let from = words.next().unwrap().parse().unwrap();
        words.next();
        let to = words.next().unwrap().parse().unwrap();
        Ok(Move { how_many, from, to })
    }
}

pub fn run_question(input: &str, question: Question) -> String {
    let mut lines_iter = input.lines();
    let cratelines: Vec<_> = lines_iter
        .take_while_ref(|l| l.trim().starts_with("["))
        .collect();
    let n_lines = cratelines.len();
    println!("{:#?}", n_lines);
    let parsed = parse_cratelines(cratelines);
    let mut stacks = lines_to_stacks(parsed);
    let indices = lines_iter.nth(0);
    println!("{:#?}", indices);
    lines_iter.next();
    let moves: Vec<Move> = lines_iter.map(|s| s.parse().unwrap()).collect();
    println!("{:#?}", moves);
    let run_fn = match question {
        Question::First => run_move_q1,
        Question::Second => run_move_q2,
    };
    for mv in moves {
        run_fn(&mut stacks, mv)
    }
    stacks
        .iter()
        .map(|st| st.last().clone().unwrap_or(&' '))
        .join("")
}

fn run_move_q1(stacks: &mut Vec<Vec<char>>, mv: Move) {
    for i in 0..(mv.how_many) {
        let ch = stacks[mv.from - 1].pop().unwrap();
        stacks[mv.to - 1].push(ch);
        println!(".")
    }
}

fn run_move_q2(stacks: &mut Vec<Vec<char>>, mv: Move) {
    // im too tired
    let mut to_push = Vec::new();
    for i in 0..(mv.how_many) {
        let ch = stacks[mv.from - 1].pop().unwrap();
        to_push.push(ch);
    }
    to_push.into_iter().rev().for_each(|ch| {
        stacks[mv.to - 1].push(ch);
    });
}
