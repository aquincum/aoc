use crate::common::day::{Day, Question};
use itertools::{repeat_n, Itertools};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day9;

impl Day for Day9 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "2333133121414131402".to_string()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Slot {
    Empty,
    File(usize),
}

impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Slot::Empty => ".".to_string(),
                Slot::File(id) if *id < 10 => id.to_string(),
                Slot::File(id) => format!("|{}|", id),
            }
        )
    }
}

struct Memory {
    slots: Vec<Slot>,
    last_non_empty: usize,
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for slot in &self.slots {
            write!(f, "{}", slot);
        }
        Ok(())
    }
}

impl Memory {
    fn set_last_non_empty(&mut self) -> Result<usize, String> {
        let item = self
            .slots
            .iter()
            .rev()
            .find_position(|sl| **sl != Slot::Empty)
            .ok_or("Only  empty slots".to_string())?;
        self.last_non_empty = self.slots.len() - item.0 - 1;
        Ok(item.0)
    }
    fn first_empty(&self) -> Option<usize> {
        self.slots
            .iter()
            .find_position(|sl| **sl == Slot::Empty)
            .map(|(pos, _)| pos)
    }
    fn first_empty_span(&self, len: usize) -> Option<usize> {
        let mut left = 0;
        let mut empties = 0;
        for i in (0..len - 1).rev() {
            if self.slots[i] == Slot::Empty {
                empties += 1;
            } else {
                break;
            }
        } // year by year I lose my functional smarts. sad :(
        while left + len < self.slots.len() {
            if self.slots[left + len - 1] == Slot::Empty {
                empties += 1;
            } else {
                empties = 0;
            }
            if empties == len {
                return Some(left);
            }
            left += 1;
        }
        None
    }
    fn move_one(&mut self) -> bool {
        // not going to functional this
        let first_empty = self.first_empty().unwrap_or(self.slots.len() + 1);
        if first_empty > self.last_non_empty {
            println!("A {} {}", first_empty, self.last_non_empty);
            return false;
        }
        self.slots[first_empty] = self.slots[self.last_non_empty];
        self.slots[self.last_non_empty] = Slot::Empty;
        if self.last_non_empty == 0 {
            return false;
        }
        self.last_non_empty -= 1;
        while self.last_non_empty >= 0 && self.slots[self.last_non_empty] == Slot::Empty {
            self.last_non_empty -= 1;
        }
        true
    }
    fn max_id(&self) -> usize {
        self.slots
            .iter()
            .map(|sl| match sl {
                Slot::Empty => 0usize,
                Slot::File(n) => *n,
            })
            .max()
            .unwrap()
    }
    fn move_file_in_one(&mut self) -> bool {
        // later
        let orig_end = self.last_non_empty;
        let file_id = match self.slots[self.last_non_empty] {
            Slot::Empty => panic!("this isn't .. supposed to happen... to me......"),
            Slot::File(id) => id,
        };
        while self.slots[self.last_non_empty] == Slot::File(file_id) {
            if self.last_non_empty == 0 {
                return false;
            }
            self.last_non_empty -= 1;
        }
        let file_length = orig_end - self.last_non_empty;
        let orig_start = self.last_non_empty + 1;

        if let Some(pos) = self.first_empty_span(file_length) {
            if pos < self.last_non_empty {
                for i in (pos..(pos + file_length)) {
                    self.slots[i] = Slot::File(file_id);
                }
                for i in (orig_start..orig_end + 1) {
                    self.slots[i] = Slot::Empty;
                }
            }
        }

        while self.last_non_empty >= 0 && self.slots[self.last_non_empty] == Slot::Empty {
            self.last_non_empty -= 1;
        }
        true
    }
    fn checksum(&self) -> u128 {
        self.slots
            .iter()
            .enumerate()
            .map(|(i, sl)| match sl {
                Slot::Empty => 0u128,
                Slot::File(id) => (i as u128) * (*id as u128),
            })
            .sum()
    }
}

impl FromStr for Memory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: Result<Vec<u32>, String> = s
            .chars()
            .map(|ch: char| ch.to_digit(10).ok_or(format!("{} is not a digit", ch)))
            .collect();
        let digits = digits?;
        let (slots, _, _) =
            digits
                .into_iter()
                .fold((vec![], false, 0), |(slot, empty, next_id), dig| {
                    if empty {
                        let newvec =
                            vec![slot, repeat_n(Slot::Empty, dig as usize).collect()].concat();
                        (newvec, false, next_id)
                    } else {
                        let newvec =
                            vec![slot, repeat_n(Slot::File(next_id), dig as usize).collect()]
                                .concat();
                        (newvec, true, next_id + 1)
                    }
                });
        let mut memory = Memory {
            slots,
            last_non_empty: 0,
        };
        memory.set_last_non_empty()?;
        Ok(memory)
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let mut memory: Memory = input.parse()?;
    if question == Question::First {
        // println!("From: {}", memory);
        while memory.move_one() {
            print!(".");
        }
    } else {
        let mut id = memory.max_id();
        while id > 0 {
            memory.move_file_in_one();
            // println!("After {}: {}", id, memory);
            // print!("         ");
            // print!("{}", " ".repeat(memory.last_non_empty));
            // println!("^");
            println!("Done {} ", id);
            id -= 1;
        }
        println!("To:      {}", memory);
    }
    // println!("To:   {}", memory);
    Ok(memory.checksum())
}
