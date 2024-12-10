use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::str::FromStr;

pub struct Day10;

impl Day for Day10 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .to_string()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Point {
    height: u8,
    reachables: Vec<(usize, usize)>,
    score: u128,
}

impl From<char> for Point {
    fn from(ch: char) -> Self {
        let height = ch.to_digit(10).unwrap() as u8;
        Point {
            height,
            reachables: vec![],
            score: 0,
        }
    }
}

struct Map(Vec<Vec<Point>>);

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|l| l.chars().map(|ch| From::from(ch)).collect_vec())
            .collect_vec()))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Map {
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }
    fn get_neighbor(&self, x: usize, y: usize, dir: Direction) -> Option<Point> {
        match (x, y, dir) {
            (0, _, Direction::Left) => None,
            (x, _, Direction::Right) if x == self.width() - 1 => None,
            (_, 0, Direction::Up) => None,
            (_, y, Direction::Down) if y == self.height() - 1 => None,
            (x, y, Direction::Up) => Some(self.0[y - 1][x].clone()),
            (x, y, Direction::Down) => Some(self.0[y + 1][x].clone()),
            (x, y, Direction::Left) => Some(self.0[y][x - 1].clone()),
            (x, y, Direction::Right) => Some(self.0[y][x + 1].clone()),
        }
    }
    fn calculate_for_height(&mut self, digit: u8) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.0[y][x].height == digit {
                    self.0[y][x].reachables = vec![
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .into_iter()
                    .map(|dir| self.get_neighbor(x, y, dir))
                    .filter(|pt| pt.is_some() && pt.clone().unwrap().height == digit + 1)
                    .map(|pt| pt.unwrap().reachables)
                    .flatten()
                    .unique()
                    .collect_vec();

                    self.0[y][x].score = vec![
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .into_iter()
                    .map(|dir| self.get_neighbor(x, y, dir))
                    .filter(|pt| pt.is_some() && pt.clone().unwrap().height == digit + 1)
                    .map(|pt| pt.unwrap().score)
                    .sum()
                }
            }
        }
    }
    fn set_scores(&mut self, digit: u8, score: u128) {
        self.0.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, p)| {
                if p.height == digit {
                    p.reachables = vec![(i, j)];
                    p.score = score;
                }
            })
        });
    }
    fn sum_reachables(&self, digit: u8) -> u128 {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|pt| match pt {
                        pt if pt.height == digit => Some(pt.reachables.len() as u128),
                        _ => None,
                    })
                    .sum::<u128>()
            })
            .sum()
    }
    fn sum_scores(&self, digit: u8) -> u128 {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|pt| match pt {
                        pt if pt.height == digit => Some(pt.score),
                        _ => None,
                    })
                    .sum::<u128>()
            })
            .sum()
    }
    fn print_reachables(&self) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                print!("{}  ", self.0[i][j].reachables.len());
            }
            println!();
        }
    }
    fn print_scores(&self) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                print!("{}  ", self.0[i][j].score);
            }
            println!();
        }
    }
}

fn q(input: &str, question: Question) -> Result<(u128, u128), String> {
    let mut map: Map = input.parse()?;
    map.set_scores(9, 1);
    for i in (0..9).rev() {
        map.calculate_for_height(i);

        println!("==== MAP AT {} ====", i);
        map.print_scores();
        println!();
    }
    Ok((map.sum_reachables(0), map.sum_scores(0)))
}
