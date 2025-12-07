use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::SystemTime;
use utf_railroad::{Choice, Diagram, NonTerminal, Terminal};

pub struct Day24;

impl Day for Day24 {
    fn question(&self, input: &str, question: Question) {
        let mut machine = Machine::from_str(input);
        if let Err(e) = machine {
            println!("Error parsing: {}", e);
            return;
        }
        if question == Question::First {
            q1(machine.unwrap());
        } else {
            q2(machine.unwrap());
        }
    }

    fn test_data(&self) -> String {
        "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
            .to_string()
    }
}

struct Machine {
    state: HashMap<String, Option<bool>>,
    gates: Vec<Gate>,
}

#[derive(Clone)]
enum Rule {
    And,
    Or,
    Xor,
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rule::And => "&",
                Rule::Or => "|",
                Rule::Xor => "^",
            }
        )
    }
}

#[derive(Clone)]
struct Gate {
    in1: String,
    in2: String,
    out: String,
    rule: Rule,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Rule::And),
            "OR" => Ok(Rule::Or),
            "XOR" => Ok(Rule::Xor),
            _ => Err(format!("{} is not a legal gate", s)),
        }
    }
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (in1, rule, in2, arrow, out) = s
            .split(" ")
            .collect_tuple()
            .ok_or(format!("{} -- not a well formed rule", s))?;
        let rule = rule.parse()?;
        Ok(Gate {
            in1: in1.to_string(),
            in2: in2.to_string(),
            out: out.to_string(),
            rule,
        })
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (initial_state, gates) = s
            .split("\n\n")
            .collect_tuple()
            .ok_or("Can't get two parts")?;
        let mut out = Machine {
            state: HashMap::new(),
            gates: vec![],
        };
        for l in initial_state.lines() {
            let (name, value) = l
                .split(": ")
                .collect_tuple()
                .ok_or(format!("Can't split {}", l))?;
            let value = match value {
                "0" => Ok(false),
                "1" => Ok(true),
                _ => Err(format!("invalid initial value {}", value)),
            }?;
            out.state.insert(name.to_string(), Some(value));
        }
        for l in gates.lines() {
            let gate = Gate::from_str(l)?;
            for register in [&gate.in1, &gate.in2, &gate.out] {
                if !out.state.contains_key(register) {
                    out.state.insert(register.clone(), None);
                }
            }
            out.gates.push(gate);
        }

        Ok(out)
    }
}

impl Rule {
    fn run(&self, in1: bool, in2: bool) -> bool {
        match self {
            Rule::And => in1 && in2,
            Rule::Or => in1 || in2,
            Rule::Xor => in1 ^ in2,
        }
    }
}

impl Machine {
    fn is_gate_runnable(&self, gate: &Gate) -> bool {
        self.state.get(&gate.in1).unwrap().is_some()
            && self.state.get(&gate.in2).unwrap().is_some()
            && self.state.get(&gate.out).unwrap().is_none()
    }
    fn run_gate(&mut self, gate: &Gate) {
        let in1 = self.state.get(&gate.in1).unwrap().unwrap();
        let in2 = self.state.get(&gate.in2).unwrap().unwrap();
        let entry = self.state.get_mut(&gate.out).unwrap();
        *entry = Some(gate.rule.run(in1, in2));
    }
    fn read_result(&self, ch: char) -> u128 {
        let zs = self
            .state
            .keys()
            .filter(|k| k.chars().nth(0).unwrap() == ch)
            .sorted()
            .collect_vec();
        let result = zs.iter().enumerate().fold(0u128, |sum, (i, x)| {
            let on = self.state.get(*x).unwrap().unwrap();
            if on {
                sum + 2u128.pow(i as u32)
            } else {
                sum
            }
        });
        result
    }
    fn run_one_set(&mut self) -> bool {
        let mut changed = false;
        for gate in &self.gates.clone() {
            if self.is_gate_runnable(gate) {
                self.run_gate(gate);
                changed = true;
            }
        }
        changed
    }

    fn print_cell(&self, cell: &str) -> (Vec<String>, HashSet<String>, Diagram) {
        let output = self.gates.iter().find(|g| g.out == cell);
        let mut levels = Vec::from([String::new()]);
        let mut involveds = HashSet::new();
        let mut diagram = Diagram::default();

        match output {
            None => {
                levels[0].push_str(&cell);
                diagram = diagram.push(Box::new(Terminal::new(&cell)));
            }
            Some(gate) => {
                levels[0].push_str(&format!("{} {} {}", gate.in1, gate.rule, gate.in2));
                let (in1_levels, in1_involveds, in1_dia) = self.print_cell(&gate.in1);
                let (in2_levels, in2_involveds, in2_dia) = self.print_cell(&gate.in2);
                let in1_depth = in1_levels.len();
                let in2_depth = in2_levels.len();
                for i in 0..(in1_depth.max(in2_depth)) {
                    let s1 = &in1_levels[if i < in1_depth { i } else { in1_depth - 1 }];
                    let s2 = &in2_levels[if i < in2_depth { i } else { in2_depth - 1 }];
                    levels.push(format!("({}) {} ({})", s1, gate.rule, s2));
                }
                let in1_first = gate.in1.chars().nth(0).unwrap();
                let in2_first = gate.in2.chars().nth(0).unwrap();
                if in1_first == 'x' || in1_first == 'y' {
                    involveds.insert(gate.in1.clone());
                }
                if in2_first == 'x' || in2_first == 'y' {
                    involveds.insert(gate.in2.clone());
                }
                involveds = involveds
                    .union(
                        &in1_involveds
                            .union(&in2_involveds)
                            .map(|s| s.clone())
                            .collect(),
                    )
                    .map(|s| s.clone())
                    .collect();
                diagram = diagram
                    .push(Box::new(Terminal::new(&gate.out)))
                    .push(Box::new(NonTerminal::new(&format! {"{}", gate.rule})))
                    .push(Box::new(
                        Choice::default()
                            .push(Box::new(in1_dia))
                            .push(Box::new(in2_dia)),
                    ))
            }
        };

        (levels, involveds, diagram)
    }
}

fn q1(mut machine: Machine) {
    let start = SystemTime::now();
    let mut changed = true;
    while changed {
        changed = machine.run_one_set();
    }
    let x = machine.read_result('x');
    let y = machine.read_result('y');
    let z = machine.read_result('z');
    println!("{} + {} =? {}", x, y, z);
    println!("{:b}", z);
    println!("{:b}", x + y);
    println!("{}ms", start.elapsed().unwrap().as_millis())
}

fn q2(mut machine: Machine) {
    let all_cells = machine.state.keys().sorted();
    // .filter(|s| s.chars().nth(0).unwrap() == 'z');
    for cell in all_cells {
        let (list, involveds, diagram) = machine.print_cell(cell);
        println!("{} = {}", cell, list.last().unwrap());

        if cell.chars().nth(0).unwrap() == 'z' {
            println!("{}", diagram);
        }
        /* println!(
            "involveds {} = {}",
            cell,
            involveds.iter().sorted().join(",")
        );*/
    }
}
