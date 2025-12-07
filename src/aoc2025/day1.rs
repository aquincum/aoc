use crate::common::day::{Day, Question};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// did in python first at work, this one doesn't work
pub struct Day1;

impl Day for Day1 {
    fn question(&self, input: &str, question: Question) {
        let turns = input.lines().map(|l| Turn::from_str(l).unwrap());
        let mut state = 50usize;
        let mut zeros_q1 = 0u128;
        let mut zeros_q2 = 0u128;
        for turn in turns {
            let (new_state, zeroes) = move_dial(&turn, state);
            state = new_state;
            zeros_q2 += zeroes as u128;
            if new_state == 0 {
                zeros_q1 += 1;
            }
        }
        println!("{} {}", zeros_q1, zeros_q2);
    }

    fn test_data(&self) -> String {
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}
struct Turn {
    direction: Direction,
    amount: usize,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = Direction::from(s.chars().nth(0).unwrap());
        let amount = s[1..].parse::<usize>().unwrap();
        Ok(Turn {
            direction: dir,
            amount,
        })
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Direction::Left => write!(f, "L{}", self.amount),
            Direction::Right => write!(f, "R{}", self.amount),
        }
    }
}

fn move_dial(turn: &Turn, x: usize) -> (usize, usize) {
    let virtual_result = match turn.direction {
        Direction::Left => x as isize - turn.amount as isize,
        Direction::Right => x as isize + turn.amount as isize,
    };
    let hundreds = virtual_result / 100;
    let hundreds = hundreds.abs() + if virtual_result < 0 { 1 } else { 0 };
    let result = if virtual_result < 0 {
        100 - (virtual_result.abs() % 100)
    } else {
        virtual_result % 100
    };
    let hundreds = hundreds
        - if x == 0 && result != 0 && turn.direction == Direction::Right && hundreds > 0 {
            1
        } else {
            0
        };
    let hundreds = if result == 0 && turn.direction == Direction::Left {
        hundreds + 1
    } else {
        hundreds
    };
    println!("Moved ({}) to {} with {} hundos", turn, result, hundreds);
    (result as usize, hundreds as usize)
}
