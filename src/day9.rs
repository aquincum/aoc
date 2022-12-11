use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::ops::{AddAssign, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

//    0123456789
// -5 ..........
// -4 .......H..
// -3 .....21...
// -2 ..543.....
// -1 .76.......
// 0  8.........
//
//    0123456789
// -5 .......H..
// -4 ......21..
// -3 ......3...
// -2 ..54......
// -1 .76.......
// 0  8.........

impl Position {
    fn zero() -> Self {
        Position { row: 0, col: 0 }
    }
    fn update(&mut self, d: Direction) {
        match d {
            Direction::Right => self.col += 1,
            Direction::Left => self.col -= 1,
            Direction::Up => self.row -= 1,
            Direction::Down => self.row += 1,
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
        (2, 2) => Position {
            row: head.row - 1,
            col: head.col - 1,
        },
        (2, -2) => Position {
            row: head.row - 1,
            col: head.col + 1,
        },
        (-2, 2) => Position {
            row: head.row + 1,
            col: head.col - 1,
        },
        (-2, -2) => Position {
            row: head.row + 1,
            col: head.col + 1,
        },

        (2, _) => Position {
            row: head.row - 1,
            col: head.col,
        },
        (-2, _) => Position {
            row: head.row + 1,
            col: head.col,
        },
        (_, 2) => Position {
            row: head.row,
            col: head.col - 1,
        },
        (_, -2) => Position {
            row: head.row,
            col: head.col + 1,
        },
        _ => panic!("yeah no. {:?} {:?} {:?}", head, tail, dist),
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
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
            _ => Err(format!("{} is unknown direction", s)),
        }
    }
}

struct InputMoves {
    direction: Direction,
    amount: i32,
}

impl FromStr for InputMoves {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        Ok(InputMoves {
            direction: parts[0].parse()?,
            amount: parts[1].parse().map_err(|e: ParseIntError| e.to_string())?,
        })
    }
}

struct InputMovesIterator {
    moves: Box<InputMoves>,
    next_idx: i32,
}

impl Iterator for InputMovesIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let mv = &self.moves;
        let rv = if self.next_idx > mv.amount {
            None
        } else {
            Some(mv.direction)
        };
        self.next_idx += 1;
        rv
    }
}

impl IntoIterator for InputMoves {
    type Item = Direction;
    type IntoIter = InputMovesIterator;

    fn into_iter(self) -> Self::IntoIter {
        InputMovesIterator {
            moves: Box::new(self),
            next_idx: 1,
        }
    }
}

pub fn question(input: &str) {
    let moves = input.split("\n").map(|l| l.parse::<InputMoves>().unwrap());
    const nodes_n: usize = 10;
    let mut nodes = [Position::zero(); nodes_n];
    let mut seen_map = HashSet::new();
    for mvs in moves {
        for dir in mvs {
            nodes[0].update(dir);
            // println!("\nMOVE: {:?}", dir);
            // println!("head: {:?}", nodes[0]);
            for i in 1..nodes_n {
                nodes[i] = new_tail(&nodes[i - 1], &nodes[i]);
                // println!("{} is at {:?}", i, nodes[i]);
            }
            seen_map.insert(nodes[nodes_n - 1]);
            // for i in -20..20 {
            //     for j in -20..20 {
            //         for k in 0..nodes_n {
            //             let pos = Position{row: i, col: j};
            //             if nodes[k] == pos {
            //                 print!("{}", k);
            //                 break;
            //             } else if k == nodes_n -1 {
            //                 print!(".");
            //             }
            //         }
            //     }
            //     print!("\n");
            // }
        }
    }
    let min_row = seen_map.iter().min_by_key(|&&k| k.row).unwrap().row;
    let max_row = seen_map.iter().max_by_key(|&&k| k.row).unwrap().row;
    let min_col = seen_map.iter().min_by_key(|&&k| k.col).unwrap().col;
    let max_col = seen_map.iter().max_by_key(|&&k| k.col).unwrap().col;
    println!(
        "Dimensions: row [{},{}] column [{},{}]",
        min_row, max_row, min_col, max_col
    );
    for i in min_row..max_row + 1 {
        for j in min_col..max_col + 1 {
            if seen_map.contains(&Position { row: i, col: j }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("{}", seen_map.len());
}
