use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day17;

impl Day for Day17 {
    fn question(&self, input: &str, question: Question) {
        let machine: Machine = input.parse().unwrap();
        let outputs = machine.run();
        println!("{}", outputs.iter().join(","));

        let mut q2_machine = machine.clone();
        // bit brute forced but the machine goes by octal digits and the worst case is it's influenced
        // by 3 digits in total, so... yeah. I tried it on pen & paper, failed tho
        for i in 0..0o1000 {
            q2_machine.state.reg_a = i + 0o6562166052247000; // + 0o61176574462000000;
            let outputs = q2_machine.run();
            if outputs.len() >= 5
                && outputs
                    .iter()
                    .rev()
                    .zip(machine.code.iter().rev())
                    .take(16)
                    .all(|(x, y)| *x == *y)
            {
                println!(
                    "{} {:o} -- {}",
                    q2_machine.state.reg_a,
                    q2_machine.state.reg_a,
                    outputs.iter().join(",")
                );
            }

            // if outputs == machine.code {
            //     println!("Found: {}", i);
            //     return;
            // }
            // if i % 100000 == 0 {
            //     println!("past {}", i);
            // }
        }
    }

    fn test_data(&self) -> String {
        "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            .to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct MachineState {
    reg_a: u128,
    reg_b: u128,
    reg_c: u128,
    ip: usize,
}

#[derive(Clone)]
struct Machine {
    state: MachineState,
    code: Vec<u8>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let reg_a = lines[0]
            .strip_prefix("Register A: ")
            .ok_or("no reg a prefix".to_string())?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let reg_b = lines[1]
            .strip_prefix("Register B: ")
            .ok_or("no reg b prefix".to_string())?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let reg_c = lines[2]
            .strip_prefix("Register C: ")
            .ok_or("no reg c prefix".to_string())?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;

        let prog_str = lines[4]
            .strip_prefix("Program: ")
            .ok_or("no prog prefix".to_string())?;
        let code: Result<Vec<u8>, String> = prog_str
            .split(",")
            .map(|s| s.parse().map_err(|e: ParseIntError| e.to_string()))
            .collect();
        Ok(Machine {
            state: MachineState {
                reg_a,
                reg_b,
                reg_c,
                ip: 0,
            },
            code: code?,
        })
    }
}

impl Machine {
    fn run(&self) -> Vec<u8> {
        let mut outputs = vec![];
        let mut state = self.state.clone();
        while state.ip < self.code.len() - 1 {
            let instruction = Instruction::new(self.code[state.ip], self.code[state.ip + 1]);
            let (new_state, output) = instruction.execute(state);
            state = new_state;
            if let Some(output) = output {
                outputs.push(output);
            }
        }
        outputs
    }
}

struct Instruction {
    command: Command,
    operand: Operand,
}

enum Command {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Operand(u8);

impl Operand {
    fn combo_value(self, state: &MachineState) -> u128 {
        match self.0 {
            n if n <= 3 => n as u128,
            4 => state.reg_a,
            5 => state.reg_b,
            6 => state.reg_c,
            _ => panic!("Illegal operand {}", self.0),
        }
    }
}

impl Instruction {
    fn new(code_i: u8, code_o: u8) -> Self {
        let command = match code_i {
            0 => Command::Adv,
            1 => Command::Bxl,
            2 => Command::Bst,
            3 => Command::Jnz,
            4 => Command::Bxc,
            5 => Command::Out,
            6 => Command::Bdv,
            7 => Command::Cdv,
            _ => panic!("Illegal command {}", code_i),
        };
        let operand = Operand(code_o);
        Instruction { command, operand }
    }
    fn execute(&self, state: MachineState) -> (MachineState, Option<u8>) {
        let mut new_state = state.clone();
        let mut output = None;
        new_state.ip += 2;
        match self.command {
            Command::Adv => {
                let op_value = self.operand.combo_value(&state);
                let num = state.reg_a;
                let denom = 2u128.pow(op_value as u32);
                new_state.reg_a = num / denom;
            }
            Command::Bxl => {
                new_state.reg_b = state.reg_b ^ (self.operand.0 as u128);
            }
            Command::Bst => {
                new_state.reg_b = self.operand.combo_value(&state) % 8;
            }
            Command::Jnz => {
                if state.reg_a != 0 {
                    new_state.ip = self.operand.0 as usize;
                }
            }
            Command::Bxc => {
                new_state.reg_b = state.reg_b ^ state.reg_c;
            }
            Command::Out => {
                let op_value = self.operand.combo_value(&state);
                output = Some((op_value % 8) as u8);
            }
            Command::Bdv => {
                let op_value = self.operand.combo_value(&state);
                let num = state.reg_a;
                let denom = 2u128.pow(op_value as u32);
                new_state.reg_b = num / denom;
            }
            Command::Cdv => {
                let op_value = self.operand.combo_value(&state);
                let num = state.reg_a;
                let denom = 2u128.pow(op_value as u32);
                new_state.reg_c = num / denom;
            }
        }
        (new_state, output)
    }
}
