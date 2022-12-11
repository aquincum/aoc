use std::num::ParseIntError;
use std::str::FromStr;
use itertools::Itertools;
use std::ops::{Div, Range};

#[derive(Copy, Clone, Debug)]
enum MonkeyOperationExpr {
    Old,
    Value(i128),
}

impl MonkeyOperationExpr {
    fn value(&self, old: i128) -> i128 {
        match self {
            MonkeyOperationExpr::Old => old,
            MonkeyOperationExpr::Value(x) => x.clone(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum MonkeyOperationOperator {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Copy, Clone, Debug)]
struct MonkeyOperation {
    left: MonkeyOperationExpr,
    op: MonkeyOperationOperator,
    right: MonkeyOperationExpr,
}

impl MonkeyOperation {
    fn execute(&self, old: i128) -> i128 {
        let left = self.left.value(old);
        let right = self.right.value(old);
        match self.op {
            MonkeyOperationOperator::Plus => left + right,
            MonkeyOperationOperator::Minus => left -right,
            MonkeyOperationOperator::Mul => left * right,
            MonkeyOperationOperator::Div => left / right,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct MonkeyTest {
    divisible: i32,
    if_false: usize,
    if_true: usize,
}

impl MonkeyTest {
    fn throw_to(&self, worry: i128) -> usize {
        if worry % (self.divisible as i128) == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    n: usize,
    items: Vec<i128>,
    operation: MonkeyOperation,
    test: MonkeyTest,
    inspected: usize,
}

// PARSING

impl FromStr for MonkeyOperationExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(MonkeyOperationExpr::Old)
        } else {
            let val = s.parse().map_err(|e: ParseIntError| e.to_string())?;
            Ok(MonkeyOperationExpr::Value(val))
        }
    }
}

impl FromStr for MonkeyOperationOperator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(MonkeyOperationOperator::Plus),
            "-" => Ok(MonkeyOperationOperator::Minus),
            "*" => Ok(MonkeyOperationOperator::Mul),
            "/" => Ok(MonkeyOperationOperator::Div),
            _ => Err(format!("{} is not a valid operator", s)),
        }
    }
}

impl FromStr for MonkeyOperation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        if words.len() != 3 {
            return Err(format!("Invalid monkey operation: {}", s));
        }
        let left = words[0].parse()?;
        let op = words[1].parse()?;
        let right = words[2].parse()?;
        Ok(MonkeyOperation { left, op, right })
    }
}

impl FromStr for MonkeyTest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(str::trim).collect::<Vec<_>>();
        if lines.len() != 3 {
            return Err(format!("Monkey test doesn't have 3 lines: {}", s));
        }
        let test_words = lines[0].split_whitespace().collect_vec();
        if test_words[0] != "Test:" || test_words[1] != "divisible" || test_words[2] != "by" {
            return Err(format!("Weird test: {}", lines[0]));
        }
        let divisible = test_words[3].parse().map_err(|e: ParseIntError| e.to_string())?;

        let true_words = lines[1].split_whitespace().collect_vec();
        if true_words[0] != "If" || true_words[1] !=  "true:" { // rest less interesting
            return Err(format!("Weird true: {}", lines[1]))
        }
        let if_true = true_words[true_words.len()-1].parse().map_err(|e: ParseIntError| e.to_string())?;

        let false_words = lines[2].split_whitespace().collect_vec();
        if false_words[0] != "If" || false_words[1] !=  "false:" { // rest less interesting
            return Err(format!("Weird false: {}", lines[1]))
        }
        let if_false = false_words[false_words.len()-1].parse().map_err(|e: ParseIntError| e.to_string())?;


        Ok(MonkeyTest{divisible, if_false, if_true })
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        if lines.len() != 6 {
            return Err(format!("Weird monkey: {}", s));
        }
        let first_line = lines[0].split_whitespace().collect_vec();
        if first_line.len() != 2 || first_line[0] != "Monkey" {
            return Err(format!("Weird first line: {}", lines[0]));
        }
        let num_str = first_line[1].strip_suffix(":").ok_or("no :".to_string())?;
        let n = num_str.parse().map_err(|e: ParseIntError| e.to_string())?;

        let items_line = lines[1].split(": ").collect_vec();
        if items_line.len() != 2 || items_line[0].trim() != "Starting items" {
            return Err(format!("Weird items line: {}", lines[1]));
        }
        let items: Result<Vec<i128>, _>= items_line[1].split(", ").map(|s| s.parse().map_err(|e: ParseIntError| e.to_string())).collect();

        let op_line = lines[2].split("Operation: new = ").collect_vec();
        if op_line.len() != 2 {
            return Err(format!("Weird operations line: {}", lines[2]));
        }
        let operation = op_line[1].parse()?;

        let test = lines[3..6].join("\n").parse()?;


        Ok(Monkey{
            n,
            items: items?,
            operation,
            test,
            inspected: 0,
        })
    }
}

fn read_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|l| l.parse().unwrap()).collect_vec()
}

pub fn question(input: &str, which_question: usize) {
    // let mut monkey = read_monkeys(input);
    //
    // for i in 0..20 {
    //     for m in 0..monkey.len() {
    //         for item in monkey[m].items.iter() {
    //             let worry = monkey[m].operation.execute(item.clone());
    //             let worry = worry / 3;
    //             let throw_to = monkey[m].test.throw_to(worry);
    //             monkey[throw_to].items.push(worry);
    //         }
    //         monkey[m].items = vec![]
    //     }
    // }
    let monkeys = read_monkeys(input);
    let alldiv = monkeys.iter().fold(1, |acc, m| acc * m.test.divisible);
    println!("Kozos: {}", alldiv);
    let range_end = if which_question == 1 { 20 } else {10000};
    let mut endmonkeys = Range{start: 0, end: range_end}.fold(monkeys.to_owned(), |round_state, i| {
        let newmonkeys = round_state.iter().fold(round_state.to_owned(), |state, m| {
            let mut newstate = state.to_owned();
            let n_items = state[m.n].items.len();
            for item in state[m.n].items.iter() {
                let worry = state[m.n].operation.execute(item.clone());
                let worry = if which_question == 1 {
                     worry / 3
                } else { worry };
                let throw_to = state[m.n].test.throw_to(worry);
                newstate[throw_to].items.push(worry % (alldiv as i128));
            }
            newstate[m.n].items = vec![];
            newstate[m.n].inspected += n_items;
            newstate
        });
        println!("ROUND {}", i);
        for monkey in newmonkeys.iter() {
            println!("Monkey {}: {} -- insp {}", monkey.n, monkey.items.iter().map(ToString::to_string).join(", "), monkey.inspected)
        }
        newmonkeys
    });
    endmonkeys.sort_by(|m, m2| m2.inspected.cmp(&m.inspected));
    for monkey in endmonkeys.iter() {
        println!("Monkey {}: {} -- insp {}", monkey.n, monkey.items.iter().map(ToString::to_string).join(", "), monkey.inspected)
    }
    println!("FINAL SCORE OF MONKEY BUSINESS: {}", endmonkeys[0].inspected * endmonkeys[1].inspected)
}