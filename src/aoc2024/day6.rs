use crate::common::day::{Day, Question};
use clap::builder::TypedValueParser;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day6;

impl Day for Day6 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string()
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Facing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Facing::Up => '^',
            Facing::Down => 'v',
            Facing::Left => '<',
            Facing::Right => '>',
        };
        write!(f, "{}", ch)
    }
}

impl Facing {
    fn turn_right(self) -> Self {
        match self {
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
        }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum Tile {
    Empty,
    Blocked,
    Start,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::Empty => '.',
            Tile::Blocked => '#',
            Tile::Start => '^',
        };
        write!(f, "{}", ch)
    }
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Tile::Empty,
            '#' => Tile::Blocked,
            '^' => Tile::Start,
            _ => panic!("unknown tile {}", ch),
        }
    }
}

struct Map {
    map: Vec<Vec<Tile>>,
    visited: Vec<Vec<HashSet<Facing>>>,
    pos: (usize, usize),
    facing: Facing,
    looping: bool,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ls = s.lines().collect_vec();
        let map: Vec<Vec<Tile>> = ls
            .into_iter()
            .map(|l| l.chars().map(|ch| ch.into()).collect_vec())
            .collect_vec();
        let starter = map
            .iter()
            .enumerate()
            .fold(None, |value, (i, row)| {
                if value.is_some() {
                    value
                } else {
                    row.iter().enumerate().fold(None, |row_found, (j, tile)| {
                        if row_found.is_some() {
                            row_found
                        } else if *tile == Tile::Start {
                            Some((i, j))
                        } else {
                            None
                        }
                    })
                }
            })
            .unwrap();
        let visited = map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| {
                        if *t == Tile::Start {
                            HashSet::from([Facing::Up])
                        } else {
                            HashSet::new()
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        Ok(Self {
            map,
            visited,
            pos: starter,
            facing: Facing::Up,
            looping: false,
        })
    }
}

impl Map {
    fn width(&self) -> usize {
        self.map[0].len()
    }
    fn height(&self) -> usize {
        self.map.len()
    }
    fn next_step(&mut self) -> bool {
        match (self.pos, self.facing) {
            ((_, 0), Facing::Left) => false,
            ((_, w), Facing::Right) if w == self.width() - 1 => false,
            ((0, _), Facing::Up) => false,
            ((h, _), Facing::Down) if h == self.height() - 1 => false,
            ((row, col), facing) => {
                let next_pos = match facing {
                    Facing::Up => (row - 1, col),
                    Facing::Down => (row + 1, col),
                    Facing::Left => (row, col - 1),
                    Facing::Right => (row, col + 1),
                };
                if self.map[next_pos.0][next_pos.1] == Tile::Blocked {
                    self.facing = self.facing.turn_right();
                } else if self.visited[next_pos.0][next_pos.1].contains(&self.facing) {
                    self.looping = true
                } else {
                    self.pos = next_pos;
                    self.visited[next_pos.0][next_pos.1].insert(self.facing);
                }
                // println!("{}", self);
                true
            }
        }
    }
    fn count_visited(&self) -> u128 {
        self.visited
            .iter()
            .map(|row| {
                row.iter().fold(
                    0u128,
                    |sum, val| if !val.is_empty() { sum + 1 } else { sum },
                )
            })
            .sum()
    }
    fn with_blockage(&self, block: (usize, usize)) -> Self {
        let map = self
            .map
            .iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles
                    .iter()
                    .enumerate()
                    .map(|(col, tile)| {
                        if block == (row, col) {
                            Tile::Blocked
                        } else {
                            tile.clone()
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        let visited = map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| {
                        if *t == Tile::Start {
                            HashSet::from([Facing::Up])
                        } else {
                            HashSet::new()
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();
        Map {
            map,
            visited,
            pos: self.pos,
            facing: self.facing,
            looping: false,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.pos == (y, x) {
                    write!(f, "{}", self.facing);
                } else if self.map[y][x] == Tile::Start {
                    write!(f, ".");
                } else {
                    write!(f, "{}", self.map[y][x]);
                }
            }
            writeln!(f);
        }
        write!(f, "")
    }
}

fn q1(input: &str) -> Result<u128, String> {
    let mut map = Map::from_str(input)?;
    println!("{}", map);
    while map.next_step() {
        println!("{} {}", map.pos.0, map.pos.1);
    }
    Ok(map.count_visited())
}

fn q2(input: &str) -> Result<u128, String> {
    let map = Map::from_str(input)?;
    let loopers = (0..map.width()).fold(0, |total_loopers, row| {
        let row_loopers = (0..map.height()).fold(0, |row_loopers, col| {
            let mut blocked_map = map.with_blockage((row, col));
            while blocked_map.next_step() {
                if blocked_map.looping {
                    println!("{} {} LOOP", row, col);
                    return row_loopers + 1;
                }
            }
            println!("{} {}", row, col);
            row_loopers
        });
        total_loopers + row_loopers
    });
    Ok(loopers)
}
