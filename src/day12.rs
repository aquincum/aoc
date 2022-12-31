use itertools::Itertools;
use std::convert::From;
use std::slice::Iter;
use std::str::FromStr;
use crate::common::day::{Day, Question};

pub struct Day12;
impl Day for Day12 {
    fn question(&self, input: &str, question: Question) {
        todo!("Implemented in go so far")
    }

    fn test_data(&self) -> String {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi".to_string()
    }
}


struct Height {
    height: i8,
    start: bool,
    end: bool,
}

struct Map(Vec<Vec<Height>>);
struct DistanceMap(Vec<Vec<Option<u32>>>);

impl From<char> for Height {
    fn from(ch: char) -> Self {
        let start = ch == 'S';
        let end = ch == 'E';
        let ch = if start {
            'a'
        } else if end {
            'z'
        } else {
            ch
        };
        Height {
            height: ch as i8 - 'a' as i8,
            start,
            end,
        }
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|l| l.chars().map(Height::from).collect_vec())
            .collect_vec()))
    }
}

impl Map {
    fn iter(&self) -> Iter<'_, Vec<Height>> {
        return self.0.iter();
    }
    fn find_start(&self) -> (usize, usize) {
        for (i, row) in self.iter().enumerate() {
            for (j, h) in row.iter().enumerate() {
                if h.start {
                    return (i, j);
                }
            }
        }
        panic!("NO START FOUND")
    }
    fn find_end(&self) -> (usize, usize) {
        for (i, row) in self.iter().enumerate() {
            for (j, h) in row.iter().enumerate() {
                if h.end {
                    return (i, j);
                }
            }
        }
        panic!("NO END FOUND")
    }
    fn size(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
}

impl DistanceMap {
    fn new(i: usize, j: usize) -> Self {
        DistanceMap(vec![vec![None; j]; i])
    }
    fn has_none(&self) -> bool {
        self.0
            .iter()
            .filter(|r| r.iter().filter(|n| n.is_none()).count() > 0)
            .count()
            > 0
    }
}

fn find_dist(map: &Map) {
    let (rows, cols) = map.size();
    let mut dist_map = DistanceMap::new(rows, cols);
    while dist_map.has_none() {} // tbc
}
