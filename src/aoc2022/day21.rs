use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::{Add, Div, Mul, Sub};
use std::str::{FromStr, Split};

type Value = i128;

type MonkeyMap = HashMap<String, Monkey>;
pub struct Solution;

impl Day for Solution {
    fn question(&self, input: &str, question: Question) {
        let mut monkeys: MonkeyMap = input
            .lines()
            .map(|l| l.parse().unwrap())
            .map(|m: Monkey| (m.name.clone(), m))
            .collect();
        match question {
            Question::First => {
                let root = monkeys.get("root").unwrap();
                let res = root.evaluate(&monkeys, Question::First);
                println!("{}", res)
            }
            Question::Second => {
                let root = monkeys.get("root").unwrap();
                let root_left = monkeys.get(&root.operation.as_ref().unwrap().left).unwrap();
                let root_right = monkeys
                    .get(&root.operation.as_ref().unwrap().right)
                    .unwrap();
                let left_res = root_left.evaluate(&monkeys, Question::Second);
                let right_res = root_right.evaluate(&monkeys, Question::Second);
                println!("{}", left_res);
                println!("{}", right_res);
            }
        }
    }

    fn test_data(&self) -> String {
        "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
            .to_string()
    }
}

#[derive(Copy, Clone, Debug)]
enum MonkeyOperationOperator {
    Plus,
    Minus,
    Mul,
    Div,
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

struct ResultValue {
    constant: f64,
    x: f64,
}

impl Add for ResultValue {
    type Output = ResultValue;

    fn add(self, rhs: Self) -> Self::Output {
        let res = ResultValue {
            constant: self.constant + rhs.constant,
            x: self.x + rhs.x,
        };
        res
    }
}
impl Sub for ResultValue {
    type Output = ResultValue;

    fn sub(self, rhs: Self) -> Self::Output {
        ResultValue {
            constant: self.constant - rhs.constant,
            x: self.x - rhs.x,
        }
    }
}

impl Mul for ResultValue {
    type Output = ResultValue;
    fn mul(self, rhs: Self) -> Self::Output {
        let res = ResultValue {
            constant: self.constant * rhs.constant,
            x: self.x * rhs.constant + self.constant * rhs.x,
        };
        if self.x > 0.0 || rhs.x > 0.0 {
            println!("*** {}*{} = {}", self, rhs, res);
        }
        if self.x > 0.0 && rhs.x > 0.0 {
            panic!("x^2 lost: {}*{} became {}", self, rhs, res);
        }
        res
    }
}

impl Div for ResultValue {
    type Output = ResultValue;

    fn div(self, rhs: Self) -> Self::Output {
        let res = match (self.x > 0.0, rhs.x > 0.0) {
            (false, false) => ResultValue {
                constant: self.constant / rhs.constant,
                x: 0.0,
            },
            (true, false) => ResultValue {
                constant: self.constant / rhs.constant,
                x: self.x / rhs.constant,
            },
            (false, true) => panic!("doesn't occur thankfully"), //ResultValue{constant: self.constant / rhs.constant, x:rhs.x / self.constant, dividend: self.constant},
            _ => panic!("doesn't occur thankfully"),
        };
        if self.x > 0.0 || rhs.x > 0.0 {
            println!("/// {} DIV {}  = {}", self, rhs, res);
        }
        res
    }
}

impl Display for ResultValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x + {}", self.x, self.constant)
    }
}

#[derive(Clone, Debug)]
struct MonkeyOperation {
    left: String,
    op: MonkeyOperationOperator,
    right: String,
}
struct Monkey {
    name: String,
    value: Option<Value>,
    operation: Option<MonkeyOperation>,
}
impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: (&str, &str) = s.split_once(": ").unwrap();
        let value = parts.1.parse::<Value>().ok();
        let operation = if value.is_none() {
            let words = parts.1.split_whitespace().collect::<Vec<_>>();
            if words.len() != 3 {
                return Err(format!("Invalid monkey operation: {}", s));
            }
            let op = words[1].parse()?;
            Some(MonkeyOperation {
                left: words[0].to_string(),
                op,
                right: words[2].to_string(),
            })
        } else {
            None
        };
        Ok(Monkey {
            name: parts.0.to_string(),
            value,
            operation,
        })
    }
}

impl MonkeyOperation {
    fn evaluate(&self, monkeys: &MonkeyMap, question: Question) -> ResultValue {
        let left = monkeys.get(&self.left).unwrap().evaluate(monkeys, question);
        let right = monkeys
            .get(&self.right)
            .unwrap()
            .evaluate(monkeys, question);
        match self.op {
            MonkeyOperationOperator::Plus => left + right,
            MonkeyOperationOperator::Minus => left - right,
            MonkeyOperationOperator::Mul => left * right,
            MonkeyOperationOperator::Div => left / right,
        }
    }
}

impl Monkey {
    fn evaluate(&self, monkeys: &MonkeyMap, question: Question) -> ResultValue {
        if question == Question::Second && self.name == "humn" {
            println!("gotteem");
            return ResultValue {
                constant: 0.0,
                x: 1.0,
            };
            // return ResultValue{constant: 3678125408017.0, x:0.0};
        }
        match &self.value {
            Some(v) => ResultValue {
                constant: v.clone() as f64,
                x: 0.0,
            },
            None => match &self.operation {
                Some(op) => op.evaluate(monkeys, question),
                None => panic!("Huh, monkey {} is weird", self.name),
            },
        }
    }
}

//q2: 3682702630750 too hi
//    3678125408017
