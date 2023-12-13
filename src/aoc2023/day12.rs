use crate::common::day::{Day, Question};
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

struct Row {
    points: Vec<Point>,
    pattern: Vec<u16>,
}

impl FromStr for Row {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (points, pattern) = s
            .split_ascii_whitespace()
            .collect_tuple()
            .ok_or("illegal row")?;
        let points = points.chars().map(|ch| Point::from(ch)).collect_vec();
        let pattern: Result<Vec<u16>, String> = pattern
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
        let allps = all_patterns(self.points.into_iter(), unknown_pipes, questions);
        allps.iter().filter(|pts| pattern_fits(&patt, *pts)).count()
    }
}

fn pattern_fits(pattern: &Vec<u16>, pipes: &Vec<Point>) -> bool {
    // let mut in_batch = false;
    // let mut curr_batch = 0;
    // let mut curr_p_in_batch = 0;
    // for p in pipes {
    //
    // }
    let (my_pattern, last) = pipes.iter().fold((vec![], 0), |(patt, curr), pipe| {
        if *pipe == Point::Pipe {
            (patt, curr + 1)
        } else {
            if curr == 0 {
                (patt, curr)
            } else {
                (vec![patt, vec![curr]].concat(), 0)
            }
        }
    });
    let my_pattern = if last == 0 {
        my_pattern
    } else {
        vec![my_pattern, vec![last]].concat()
    };
    my_pattern
        .iter()
        .enumerate()
        .all(|(i, x)| x == pattern.get(i).unwrap_or(&0))
}

// n unknown pipes with k question marks
fn all_patterns<I>(mut p: I, n: usize, k: usize) -> Vec<Vec<Point>>
where
    I: Iterator<Item = Point>,
{
    let next = p.next();
    if let Some(next) = next {
        match next {
            Point::Ground | Point::Pipe => vec![vec![vec![next]], all_patterns(p, n, k)].concat(),
            Point::Unknown => {
                if n == 0 {
                    vec![vec![vec![Point::Ground]], all_patterns(p, n, k)].concat()
                } else if n == k {
                    vec![vec![vec![Point::Pipe]], all_patterns(p, n, k)].concat()
                } else {
                    let pall = p.collect_vec();
                    vec![
                        vec![
                            vec![vec![Point::Pipe]],
                            all_patterns(pall.iter().map(|x| *x), n - 1, k),
                        ]
                        .concat(),
                        vec![
                            vec![vec![Point::Ground]],
                            all_patterns(pall.iter().map(|x| *x), n, k),
                        ]
                        .concat(),
                    ]
                    .concat()
                }
            }
        }
    } else {
        vec![]
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let rows: Result<Vec<Row>, String> = input.lines().map(|l| l.parse()).collect();
    let rows = rows?;
    Ok(rows.into_iter().map(|r| r.variations() as u128).sum())
}
