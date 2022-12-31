use itertools::Itertools;
use crate::common::day::{Question, Day};

const DECRYPTION_CONSTANT: i128 = 811589153;
pub const DEMO_INPUT: &str = "1
2
-3
3
-2
0
4";

pub struct Day20;
impl Day for Day20 {
    fn question(&self, input: &str, question: Question) {
        self::question(input, question);
    }

    fn test_data(&self) -> String {
        DEMO_INPUT.to_string()
    }
}

struct Num {
    orig_value: i128,
    value: i128,
    positions: Vec<i32>,
}

impl Num {
    fn new(idx: usize, value: i128) -> Self {
        Num {
            orig_value: value,
            value,
            positions: vec![idx as i32]
        }
    }
    fn pos(&self) -> i32 {
        self.positions.last().unwrap().clone() as i32
    }
    fn push(&mut self, n: i32) {
        self.positions.push(n);
    }
}

trait Vecnum {
    fn print_state(&self);
    fn get_nth(&self,n: usize) -> i128;
}


impl Vecnum for Vec<Num> {
    fn print_state(&self) {
        let mut nums_to_sort = self.iter().collect_vec();
        nums_to_sort.sort_by_key(|n| n.pos());
        println!("{}", nums_to_sort.iter().map(|n| format!("{}",n.value)).join(", "));
    }
    fn get_nth(&self, n: usize) -> i128 {
        let null_pos = self.iter().find(|n| n.value == 0).unwrap();
        let pos = null_pos.pos() + (n as i32);
        let pos = pos % (self.len() as i32);
        self.iter().find(|n| n.pos() == pos).map(|n| n.orig_value).unwrap()
    }
}

pub fn question(input: &str, question: Question) {
    let mut nums: Vec<Num> = input.lines().map(|l| l.parse().unwrap()).enumerate().map(|(i, v)| Num::new(i, v)).collect_vec();
    if question == Question::Second {
        for i in 0..nums.len() {
            nums[i].orig_value = nums[i].value  * DECRYPTION_CONSTANT;
            nums[i].value = (nums[i].value * DECRYPTION_CONSTANT) % ((nums.len()-1) as i128);
        }
    }
    let rounds = match question {
        Question::First => 1i8,
        Question::Second => 10i8,
    };
    for round in 0..rounds {
        for i in 0..nums.len() {
            let old_pos = nums[i].pos() as i128;
            let mut new_pos = (old_pos as i128) + nums[i].value;
            // println!("A{}", new_pos);
            while new_pos > nums.len() as i128 {
                new_pos -= (nums.len() - 1) as i128;
            }
            while new_pos <= 0 {
                new_pos += (nums.len() - 1) as i128;
                // println!("B{}", new_pos);
            }
            for j in 0..nums.len() {
                if i == j {
                    nums[i].push(new_pos as i32);
                    // println!("New pos for {} is {}", nums[i].value, new_pos);
                } else {
                    let j_pos = nums[j].pos() as i128;
                    if j_pos >= new_pos && j_pos < old_pos {
                        nums[j].push((j_pos as i32) + 1);
                    } else if j_pos <= new_pos && j_pos > old_pos {
                        nums[j].push((j_pos as i32) - 1);
                    } else {
                        nums[j].push((j_pos as i32));
                    }
                }
            }
            // nums.print_state();
        }
        println!("Done with round {}", round)
    }
    let thou = nums.get_nth(1000);
    println!("1000: {}", thou);
    let twothou = nums.get_nth(2000);
    println!("2000: {}", twothou);
    let threethou = nums.get_nth(3000);
    println!("3000: {}", threethou);
    println!("RESULT: {}", thou + twothou + threethou);
    // 5743 too lo
    // 11387407405743 too hi
    // 5100837826605 ain ri
}