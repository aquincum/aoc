use crate::common::day::{Day, Question};
use itertools::Itertools;

pub struct Solution;

impl Day for Solution {
    fn question(&self, input: &str, question: Question) {
        todo!()
    }

    fn test_data(&self) -> String {
        "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
            .to_string()
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum Pixel {
    Open,
    Wall,
    Nothing,
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Location {
    facing: Facing,
    x: usize,
    y: usize,
}

impl From<char> for Pixel {
    fn from(ch: char) -> Self {
        match ch {
            ' ' => Pixel::Nothing,
            '#' => Pixel::Wall,
            '.' => Pixel::Open,
            _ => panic!("Nonexistent map tile {}", ch),
        }
    }
}

struct Map {
    map: Vec<Vec<Pixel>>,
}

impl Map {
    fn new(s: &str) -> Self {
        let ls = s.lines().collect::<Vec<_>>();
        let map = ls
            .iter()
            .map(|l| l.chars().map(|ch| ch.into()).collect_vec())
            .collect_vec();
        Self { map }
    }
    fn first_open_column(&self) -> usize {
        self.map[0]
            .iter()
            .enumerate()
            .find(|(i, p)| **p == Pixel::Open)
            .map(|(i, _)| i)
            .unwrap()
    }
    fn max_y(&self) -> usize {
        self.map.len() - 1
    }
    fn max_x(&self) -> usize {
        self.map[0].len() - 1
    }
    fn move_one(&self, loc: Location) -> Location {
        let mut new_xy = (loc.x, loc.y);
        loop {
            new_xy = match loc.facing {
                Facing::Up => {
                    if loc.y == 0 {
                        (loc.x, self.max_y())
                    } else {
                        (loc.x, loc.y - 1)
                    }
                }
                Facing::Down => {
                    if loc.y == self.max_y() {
                        (loc.x, 0)
                    } else {
                        (loc.x, loc.y + 1)
                    }
                }
                Facing::Left => {
                    if loc.x == 0 {
                        (self.max_x(), loc.y)
                    } else {
                        (loc.x - 1, loc.y)
                    }
                }
                Facing::Right => {
                    if loc.x == self.max_x() {
                        (0, loc.y)
                    } else {
                        (loc.x + 1, loc.y)
                    }
                }
            };
            if self.map[new_xy.0][new_xy.1] == Pixel::Wall {
                return loc;
            }
            if self.map[new_xy.0][new_xy.1] == Pixel::Open {
                return Location {
                    facing: loc.facing,
                    x: new_xy.0,
                    y: new_xy.1,
                };
            }
        }
    }
}
