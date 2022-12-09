use std::ops::{Sub, AddAssign};
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    row: i32,
    col: i32
}

impl Position {
    fn zero() -> Self {
        Position{row: 0, col: 0}
    }
    fn update(&mut self, d: Direction) {
        match d {
            Direction::Right => self.col += 1,
            Direction::Left => self.col -= 1,
            Direction::Up => self.row += 1,
            Direction::Down => self.row -= 1,
        }
    }
}



impl Sub for &Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

fn new_tail(head: &Position, tail: &Position) -> Position {
    let dist = head - tail;
    if dist.row.abs() <= 1 && dist.col.abs() <= 1 {
        return tail.clone();
    }
    match (dist.row, dist.col) {
        (2,_) => Position{ row: head.row-1, col: head.col },
        (-2,_) => Position{ row: head.row+1, col: head.col },
        (_,2) => Position{ row: head.row, col: head.col-1 },
        (_,-2) => Position{ row: head.row, col: head.col+1 },
        _ => panic!("yeah no. {:?} {:?} {:?}", head, tail, dist)
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right, Left, Up, Down
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::day9::Direction::*;
        match s {
            "R" => Ok(Right),
            "L" => Ok(Left),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => Err(format!("{} is unknown direction", s))
        }
    }
}

struct InputMoves {
    direction: Direction,
    amount: i32,
}

impl FromStr for InputMoves{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        Ok(InputMoves{
            direction: parts[0].parse()?,
            amount: parts[1].parse().map_err(|e: ParseIntError| e.to_string())?,
        })
    }
}

struct InputMovesIterator {
    moves: Box<InputMoves>,
    next_idx: i32,
}

impl Iterator for InputMovesIterator{
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let mv = &self.moves;
        let rv = if self.next_idx > mv.amount {
            None
        } else {
            Some(mv.direction)
        };
        self.next_idx+=1;
        rv
    }
}

impl IntoIterator for InputMoves{
    type Item = Direction;
    type IntoIter = InputMovesIterator;

    fn into_iter(self) -> Self::IntoIter {
        InputMovesIterator{
            moves: Box::new(self),
            next_idx: 1,
        }
    }
}

pub fn question(input: &str){
    let moves = input.split("\n").map(|l| l.parse::<InputMoves>().unwrap());
    let mut head = Position::zero();
    let mut tail = Position::zero();
    let mut seen_map = HashSet::new();
    for mvs in moves {
        for dir in mvs {
            head.update(dir);
            tail = new_tail(&head, &tail);
            seen_map.insert(tail);
        }
    }
    println!("{}",seen_map.len());
}
