use crate::common::day::{Day, Question};
use crate::common::map::Map;
use serde_json::de::Read;
use std::ops::Range;

pub struct Day4;

impl Day for Day4 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string()
    }
}

#[derive(Debug)]
enum State {
    X,
    M,
    A,
    S,
}

enum Pointer {
    Point(usize, usize),
    EndOfRow,
}

struct RowWalkthrough {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Iterator for RowWalkthrough {
    type Item = Pointer;

    fn next(&mut self) -> Option<Self::Item> {
        let current = Pointer::Point(self.x, self.y);
        if self.y == self.height {
            return None;
        }
        if self.x == self.width {
            self.x = 0;
            self.y += 1;
            return Some(Pointer::EndOfRow);
        }
        self.x += 1;
        return Some(current);
    }
}

struct DiagonalWalkthrough {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Iterator for DiagonalWalkthrough {
    type Item = Pointer;

    fn next(&mut self) -> Option<Self::Item> {
        let current = Pointer::Point(self.x, self.y);
        // assuming square
        if self.y == self.height && self.x == 0 {
            return None;
        }
        if self.y == self.height {
            self.y = self.height - self.x + 1;
            self.x = 0;
            return Some(Pointer::EndOfRow);
        }
        if self.x == self.width {
            self.x = self.width - self.y - 1;
            self.y = 0;
            return Some(Pointer::EndOfRow);
        }
        self.x += 1;
        self.y += 1;
        return Some(current);
    }
}

fn walk_map<I>(map: &Map<char>, iter: I) -> u128
where
    I: Iterator<Item = Pointer>,
{
    println!("=====================");
    let (res, _, _) = iter.fold(
        (0u128, State::X, State::S),
        |(finds, state, revstate), pointer| match pointer {
            Pointer::Point(x, y) => {
                let point_ch = map.get_no_floor(x, y);
                println!(
                    "{} {} {} hoping for {:?} rev {:?}",
                    x, y, point_ch, state, revstate
                );
                let (finds, state) = match (point_ch, state) {
                    ('X', _) => (finds, State::M),
                    ('M', State::M) => (finds, State::A),
                    ('A', State::A) => (finds, State::S),
                    ('S', State::S) => {
                        println!("OLE!");
                        (finds + 1, State::X)
                    }
                    _ => (finds, State::X),
                };
                let (finds, revstate) = match (point_ch, revstate) {
                    ('S', _) => (finds, State::A),
                    ('A', State::A) => (finds, State::M),
                    ('M', State::M) => (finds, State::X),
                    ('X', State::X) => {
                        println!("REVOLE!");
                        (finds + 1, State::S)
                    }
                    _ => (finds, State::S),
                };
                (finds, state, revstate)
            }
            Pointer::EndOfRow => (finds, State::X, State::S),
        },
    );
    println!("RES {}", res);
    res
}

fn q1(input: &str) -> Result<u128, String> {
    let map: Map<char> = input.parse()?;
    let rot_map = map.rotate();
    println!("{}", map);
    println!("======\n{}", rot_map);
    Ok(walk_map(
        &map,
        RowWalkthrough {
            x: 0,
            y: 0,
            width: map.width(),
            height: map.height(),
        },
    ) + walk_map(
        &rot_map,
        RowWalkthrough {
            x: 0,
            y: 0,
            width: map.height(),
            height: map.width(),
        },
    ) + walk_map(
        &map,
        DiagonalWalkthrough {
            x: map.width() - 1,
            y: 0,
            width: map.width(),
            height: map.height(),
        },
    ) + walk_map(
        &rot_map,
        DiagonalWalkthrough {
            x: map.height() - 1,
            y: 0,
            width: map.height(),
            height: map.width(),
        },
    ))
}

fn q2(input: &str) -> Result<u128, String> {
    let map: Map<char> = input.parse()?;
    let mut found_without_thinking = 0u128;
    for x in 0..(map.width() - 2) {
        for y in 0..(map.height() - 2) {
            let test = format!(
                "{}{}{}{}{}",
                map.get_no_floor(x, y),
                map.get_no_floor(x + 2, y),
                map.get_no_floor(x + 1, y + 1),
                map.get_no_floor(x, y + 2),
                map.get_no_floor(x + 2, y + 2)
            );
            println!("{}", test);
            if test == "MMASS" || test == "MSAMS" || test == "SMASM" || test == "SSAMM" {
                found_without_thinking += 1;
            }
        }
    }
    Ok(found_without_thinking)
}

// M M   M S   S M    S S
//  A     A     A      A
// S S   M S   S M    M M
