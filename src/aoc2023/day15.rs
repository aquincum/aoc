use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::iter::repeat;
use std::num::ParseIntError;
use std::str::FromStr;
use std::string::FromUtf8Error;

pub struct Day15;

impl Day for Day15 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string()
    }
}

fn hasher(s: &str) -> u16 {
    s.chars().fold(0, |val, ch| {
        let asc = ch as u16;
        let val = asc + val;
        let val = val * 17;
        let val = val % 256;
        val
    })
}

fn q1(input: &str) -> Result<u128, String> {
    let steps = input.split(",");
    let vals = steps.map(|s| hasher(s) as u128);
    Ok(vals.sum())
}

#[derive(Clone)]
struct LabeledLens {
    label: String,
    lens: u8,
}

enum Task {
    Add(LabeledLens),
    Remove(String),
}

impl FromStr for Task {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s.len();
        if s.len() == 0 {
            Err("zero length string".to_string())
        } else if let Some(label) = s.strip_suffix("-") {
            Ok(Task::Remove(label.to_string()))
        } else {
            let (label, lens) = s
                .split("=")
                .collect_tuple()
                .ok_or(format!("Can't split {} into two", s))?;
            let lens = lens.parse().map_err(|e: ParseIntError| e.to_string())?;
            Ok(Task::Add(LabeledLens {
                label: label.to_string(),
                lens,
            }))
        }
    }
}

fn q2(input: &str) -> Result<u128, String> {
    let steps = input.split(",");
    let tasks: Result<Vec<Task>, String> = steps.map(|t| t.parse()).collect();
    let tasks = tasks?;
    let mut boxes: Vec<Vec<LabeledLens>> = repeat(vec![]).take(256).collect_vec();
    for task in tasks {
        match task {
            Task::Add(LabeledLens { label, lens }) => {
                let box_id = hasher(&label);
                let the_box = boxes
                    .get_mut(box_id as usize)
                    .ok_or(format!("No box {} for {} add", box_id, label))?;
                if let Some(pos) = the_box.iter().position(|ll| ll.label == label) {
                    the_box[pos].lens = lens;
                } else {
                    the_box.push(LabeledLens { label, lens })
                }
            }
            Task::Remove(label) => {
                let box_id = hasher(&label);
                let the_box = boxes
                    .get_mut(box_id as usize)
                    .ok_or(format!("No box {} for {} remove", box_id, label))?;
                if let Some(pos) = the_box.iter().position(|ll| ll.label == label) {
                    the_box.remove(pos);
                }
            }
        }
    }
    let box_powers = boxes
        .iter()
        .enumerate()
        .map(|(box_id, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_order, lens)| {
                    ((box_id + 1) * (lens_order + 1) * (lens.lens as usize)) as u128
                })
                .sum::<u128>()
        })
        .sum::<u128>();
    Ok(box_powers)
}
