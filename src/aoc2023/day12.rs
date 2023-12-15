use crate::common::day::{Day, Question};
use bit_vec::BitVec;
use itertools::{all, Itertools};
use std::num::ParseIntError;
use std::str::FromStr;

// TODO rewrite, this doesn't work
pub struct Day12;

impl Day for Day12 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res)
    }

    fn test_data(&self) -> String {
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            .to_string()
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Point {
    Ground,
    Pipe,
    Unknown,
}

impl From<char> for Point {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Point::Pipe,
            '?' => Point::Unknown,
            _ => Point::Ground,
        }
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        match self {
            Point::Ground => ".".to_string(),
            Point::Pipe => "#".to_string(),
            Point::Unknown => "?".to_string(),
        }
    }
}

struct Row {
    points: Vec<Point>,
    pattern: Vec<usize>,
}

impl FromStr for Row {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (points, pattern) = s
            .split_ascii_whitespace()
            .collect_tuple()
            .ok_or("illegal row")?;
        let points = points.chars().map(|ch| Point::from(ch)).collect_vec();
        let pattern: Result<Vec<usize>, String> = pattern
            .split(",")
            .map(|n| n.parse().map_err(|e: ParseIntError| e.to_string()))
            .collect();
        let pattern = pattern?;
        Ok(Row { points, pattern })
    }
}

impl Row {
    fn variations(self) -> usize {
        let all_pipes: usize = self.pattern.iter().map(|x| *x as usize).sum();
        let (existing_pipes, questions) = self.points.iter().fold((0, 0), |(x, q), p| match p {
            Point::Ground => (x, q),
            Point::Pipe => (x + 1, q),
            Point::Unknown => (x, q + 1),
        });
        let unknown_pipes = all_pipes - existing_pipes;
        let patt = self.pattern.into_iter().collect();
        let allps = all_patterns(self.points, unknown_pipes, questions);
        allps.iter().filter(|pts| pattern_fits(&patt, *pts)).count()
    }
    fn times_five(self) -> Self {
        let pattern = self.pattern.repeat(5);
        let points = vec![
            &self.points,
            &self.points,
            &self.points,
            &self.points,
            &self.points,
        ]
        .into_iter()
        .intersperse(&vec![Point::Unknown])
        .flatten()
        .map(|pt| *pt)
        .collect_vec();
        Row { pattern, points }
    }
}

fn pattern_fits(pattern: &Vec<usize>, pipes: &Vec<Point>) -> bool {
    let mut curr_patt = 0;
    for (key, group) in &pipes.iter().group_by(|pipe| **pipe) {
        if key == Point::Pipe {
            if curr_patt >= pattern.len() {
                return false;
            }
            if pattern[curr_patt] != group.count() {
                return false;
            }
            curr_patt += 1;
        }
    }
    true
}

// n unknown pipes with k question marks
fn all_patterns(p: Vec<Point>, n: usize, k: usize) -> Vec<Vec<Point>> {
    (0..(2u32.pow(k as u32)))
        .filter(|i| count_bits(*i) == n as u32)
        .map(|mut bit_pattern| {
            p.iter()
                .map(|pt| {
                    if pt == &Point::Unknown {
                        let new_pipe = if bit_pattern & 1 == 1 {
                            Point::Pipe
                        } else {
                            Point::Ground
                        };
                        bit_pattern >>= 1;
                        new_pipe
                    } else {
                        pt.clone()
                    }
                })
                .collect_vec()
        })
        .collect_vec()
}

fn count_bits(n: u32) -> u32 {
    let norig = n;
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let rows: Result<Vec<Row>, String> = input.lines().map(|l| l.parse()).collect();
    let rows = rows?;
    let n = rows.len();
    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(idx, r)| {
            let s = match question {
                Question::First => r.variations() as u128,
                Question::Second => r.times_five().variations() as u128,
            };
            println!("{}/{}", idx, n);
            s
        })
        .sum())
}
