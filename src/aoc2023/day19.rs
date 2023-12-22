use crate::common::day::{Day, Question};
use itertools::Itertools;
use regex::Regex;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day19;

impl Day for Day19 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            .to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Measurement {
    X,
    M,
    A,
    S,
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Action {
    Send(String),
    Reject,
    Accept,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]

enum Condition {
    Always,
    GreaterThan(Measurement, usize),
    LessThan(Measurement, usize),
}

#[derive(Debug)]
struct ConditionIfClause {
    condition: Condition,
    action: Action,
}

struct Workflow {
    name: String,
    ifs: Vec<ConditionIfClause>,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();
        let caps = re.captures(s).ok_or(format!("No capture for part {}", s))?;
        let extr = |i: usize| -> Result<usize, String> {
            caps.get(i)
                .ok_or(format!("no capture for {}", i))?
                .as_str()
                .parse::<usize>()
                .map_err(|e: ParseIntError| e.to_string())
        };
        let x = extr(1)?;
        let m = extr(2)?;
        let a = extr(3)?;
        let s = extr(4)?;
        Ok(Part { x, m, a, s })
    }
}

impl Part {
    fn sum(&self) -> u128 {
        (self.x + self.m + self.a + self.s) as u128
    }
    fn get(&self, measurement: &Measurement) -> usize {
        match measurement {
            Measurement::X => self.x,
            Measurement::M => self.m,
            Measurement::A => self.a,
            Measurement::S => self.s,
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Action::Accept),
            "R" => Ok(Action::Reject),
            s => Ok(Action::Send(s.to_string())),
        }
    }
}

impl FromStr for Condition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let measurement = match s.chars().nth(0).ok_or(" no chars")? {
            'x' => Measurement::X,
            'm' => Measurement::M,
            'a' => Measurement::A,
            's' => Measurement::S,
            _ => Err(format!("not a valid measurement for {}", s))?,
        };
        let value = s
            .chars()
            .skip(2)
            .collect::<String>()
            .parse::<usize>()
            .map_err(|e: ParseIntError| e.to_string())?;
        let condition = match s.chars().nth(1).ok_or(format!("{} is too short", s))? {
            '<' => Condition::LessThan(measurement, value),
            '>' => Condition::GreaterThan(measurement, value),
            _ => Err(format!("Invalid operator for {}", s))?,
        };
        Ok(condition)
    }
}

impl FromStr for ConditionIfClause {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((condition, action)) = s.split(":").collect_tuple() {
            let condition = condition.parse()?;
            let action = action.parse()?;
            Ok(ConditionIfClause { condition, action })
        } else {
            Ok(ConditionIfClause {
                condition: Condition::Always,
                action: s.parse()?,
            })
        }
    }
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([a-z]+)\{([^{]+)\}").unwrap();
        let caps = re
            .captures(s)
            .ok_or(format!("No capture for workflow {}", s))?;
        let name = caps
            .get(1)
            .ok_or(format!("{} has no name", s))?
            .as_str()
            .to_string();
        let ifs = caps
            .get(2)
            .ok_or(format!("{} has no workflow", s))?
            .as_str()
            .split(",")
            .map(|s| s.parse::<ConditionIfClause>())
            .collect::<Result<Vec<ConditionIfClause>, String>>()?;
        Ok(Workflow { name, ifs })
    }
}

fn parse_input(input: &str) -> Result<(Vec<Workflow>, Vec<Part>), String> {
    let (workflows, parts) = input
        .split("\n\n")
        .collect_tuple()
        .ok_or("No double newline")?;
    let workflows = workflows
        .split("\n")
        .map(|s| s.parse())
        .collect::<Result<Vec<Workflow>, String>>()?;
    let parts = parts
        .split("\n")
        .map(|s| s.parse())
        .collect::<Result<Vec<Part>, String>>()?;
    Ok((workflows, parts))
}

fn process_part(wfmap: &HashMap<String, Workflow>, part: &Part) -> Result<Action, String> {
    let mut current = "in".to_string();
    'workflows: loop {
        let workflow = wfmap
            .get(&current)
            .ok_or(format!("Workflow {} not found", current))?;
        for cic in workflow.ifs.iter() {
            println!("In {:?}", cic);
            let action = match cic.condition.clone() {
                Condition::Always => Some(cic.action.clone()),
                Condition::GreaterThan(meas, value) => {
                    let part_value = part.get(&meas);
                    if part_value > value {
                        Some(cic.action.clone())
                    } else {
                        None
                    }
                }
                Condition::LessThan(meas, value) => {
                    let part_value = part.get(&meas);
                    if part_value < value {
                        Some(cic.action.clone())
                    } else {
                        None
                    }
                }
            };
            if let Some(action) = action {
                match action {
                    Action::Send(s) => {
                        current = s;
                        continue 'workflows;
                    }
                    a => return Ok(a),
                }
            }
        }
        return Err(format!("no resolution for {:?} in {}", part, workflow.name));
    }
}

fn q1(input: &str) -> Result<u128, String> {
    let (workflows, parts) = parse_input(input)?;
    let workflow_map: HashMap<String, Workflow> = build_workflow_map(workflows);

    let accepteds = parts
        .iter()
        .filter(|&part| process_part(&workflow_map, part).unwrap() == Action::Accept);
    Ok(accepteds.map(|part| part.sum()).sum())
}

fn build_workflow_map(workflows: Vec<Workflow>) -> HashMap<String, Workflow> {
    workflows
        .into_iter()
        .map(|wf| (wf.name.clone(), wf))
        .collect()
}

#[derive(Copy, Clone, Debug)]
struct PartSpan {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl PartSpan {
    fn get(&self, measurement: &Measurement) -> (usize, usize) {
        match measurement {
            Measurement::X => self.x,
            Measurement::M => self.m,
            Measurement::A => self.a,
            Measurement::S => self.s,
        }
    }

    fn set(&mut self, measurement: Measurement, value: (usize, usize)) {
        match measurement {
            Measurement::X => self.x = value,
            Measurement::M => self.m = value,
            Measurement::A => self.a = value,
            Measurement::S => self.s = value,
        }
    }

    fn split_at(
        &self,
        measurement: Measurement,
        value: usize,
        goes_with_min: bool,
    ) -> (PartSpan, Option<PartSpan>) {
        let (min, max) = self.get(&measurement);
        if value > min && value < max {
            let mut a = *self;
            let mut b = *self;
            if goes_with_min {
                b.set(measurement, (min, value));
                a.set(measurement, (value + 1, max));
            } else {
                a.set(measurement, (min, value - 1));
                b.set(measurement, (value, max));
            }
            (a, Some(b))
        } else {
            (*self, None)
        }
    }

    fn combinations(&self) -> u128 {
        let x = (self.x.1 - self.x.0 + 1) as u128;
        let m = (self.m.1 - self.m.0 + 1) as u128;
        let a = (self.a.1 - self.a.0 + 1) as u128;
        let s = (self.s.1 - self.s.0 + 1) as u128;
        x * m * a * s
    }
}

struct Work {
    workflow: String,
    cond_idx: usize,
    spans: PartSpan,
}

fn q2(input: &str) -> Result<u128, String> {
    let (workflows, parts) = parse_input(input)?;
    let workflow_map: HashMap<String, Workflow> = build_workflow_map(workflows);
    let mut work_queue = vec![Work {
        workflow: "in".to_string(),
        cond_idx: 0,
        spans: PartSpan {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    }];
    let mut win_conditions = vec![];
    while let Some(work) = work_queue.pop() {
        let cic = workflow_map
            .get(&work.workflow)
            .unwrap()
            .ifs
            .get(work.cond_idx)
            .unwrap();
        let (my_span, alt_span) = match cic.condition {
            Condition::Always => (work.spans, None),
            Condition::GreaterThan(m, x) => work.spans.split_at(m, x, true),
            Condition::LessThan(m, x) => work.spans.split_at(m, x, false),
        };
        if let Some(alt_span) = alt_span {
            work_queue.push(Work {
                workflow: work.workflow.clone(),
                cond_idx: work.cond_idx + 1,
                spans: alt_span,
            })
        }
        match cic.action.clone() {
            Action::Send(s) => work_queue.push(Work {
                workflow: s.clone(),
                cond_idx: 0,
                spans: my_span,
            }),
            Action::Reject => {}
            Action::Accept => win_conditions.push(my_span),
        }
    }
    for winning_span in &win_conditions {
        println!("{:?}", winning_span)
    }
    Ok(win_conditions.iter().map(|wc| wc.combinations()).sum())
}
