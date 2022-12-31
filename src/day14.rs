use crate::common::map::{Bottom, Map};
use crate::common::day::{Question, Day};
use itertools::Itertools;
use serde_json::ser::CharEscape::LineFeed;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::str::Split;

pub struct Day14;
impl Day for Day14 {
    fn question(&self, input: &str, question: Question) {
        self::question(input, question);
    }

    fn test_data(&self) -> String {
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9".to_string()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Object {
    Air,
    Rock,
    Sand,
}

impl Default for Object {
    fn default() -> Self {
        Object::Air
    }
}

impl Bottom for Object {
    fn bottom() -> Self {
        Object::Rock
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Object::Air => ".",
            Object::Rock => "#",
            Object::Sand => "o",
        };
        write!(f, "{}", symbol)
    }
}

struct Line {
    from: (usize, usize),
    to: (usize, usize),
}

impl Line {
    fn min_x(&self) -> usize {
        if self.from.0 < self.to.0 {
            self.from.0
        } else {
            self.to.0
        }
    }
    fn min_y(&self) -> usize {
        if self.from.1 < self.to.1 {
            self.from.1
        } else {
            self.to.1
        }
    }
    fn max_x(&self) -> usize {
        if self.from.0 < self.to.0 {
            self.to.0
        } else {
            self.from.0
        }
    }
    fn max_y(&self) -> usize {
        if self.from.1 < self.to.1 {
            self.to.1
        } else {
            self.from.1
        }
    }
    fn put_on_map(&self, map: &mut Map<Object>) {
        for x in self.min_x()..(self.max_x() + 1) {
            for y in self.min_y()..(self.max_y() + 1) {
                map.set(x, y, Object::Rock);
            }
        }
    }
}

fn parse_lines(input: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    let input_rows = input.split("\n");
    for row in input_rows {
        let vertices: Vec<(usize, usize)> = row
            .split(" -> ")
            .map(|s| s.split_once(",").unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect_vec();
        let mut last_to: Option<(usize, usize)> = None;
        for vertex in vertices {
            if let Some(l) = last_to {
                lines.push(Line {
                    from: l,
                    to: vertex,
                });
            }
            last_to = Some(vertex);
        }
    }
    lines
}

pub fn question(input: &str, which_question: Question) {
    let lines = parse_lines(input);
    let min_x = lines.iter().min_by_key(|l| l.min_x()).unwrap().min_x() - 150;
    let max_x = lines.iter().max_by_key(|l| l.max_x()).unwrap().max_x() + 150;
    let max_y = lines.iter().max_by_key(|l| l.max_y()).unwrap().max_y() + 1;
    let mut map = Map::new(min_x, max_x, max_y, which_question == Question::Second);
    lines.iter().for_each(|l| l.put_on_map(&mut map));
    println!("{}", map);
    let mut cnt: usize = 0;

    while !map.drop(500, 0) {
        cnt += 1;
        if map.get(500, 0) == Object::Sand {
            break;
        }
        if cnt % 500 == 0 {
            print!(".");
        }
    }
    println!("{}", map);
    println!("\n{}", cnt)
}
