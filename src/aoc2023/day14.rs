use crate::common::day::{Day, Question};
use clap::ValueHint::DirPath;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ptr::write;
use std::str::FromStr;

pub struct Day14;

impl Day for Day14 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            .to_string()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Piece {
    O,
    Wall,
    Ground,
}

impl From<char> for Piece {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Piece::Wall,
            'O' => Piece::O,
            _ => Piece::Ground,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::O => write!(f, "O"),
            Piece::Wall => write!(f, "#"),
            Piece::Ground => write!(f, "."),
        }
    }
}

struct Map(Vec<Vec<Piece>>);

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|line| line.chars().map(|ch| Piece::from(ch)).collect_vec())
            .collect_vec()))
    }
}

#[derive(Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn moved(&self, i: usize, j: usize) -> (usize, usize) {
        match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }
    fn next(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

impl Map {
    fn bounds_check(&self, direction: &Direction, i: usize, j: usize) -> bool {
        match direction {
            Direction::Up => i != 0,
            Direction::Down => i != self.height() - 1,
            Direction::Left => j != 0,
            Direction::Right => j != self.width() - 1,
        }
    }
    fn tilt(&mut self, direction: &Direction) -> bool {
        let mut changed = false;
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.0[i][j] == Piece::O && self.bounds_check(direction, i, j) {
                    let (nexti, nextj) = direction.moved(i, j);
                    match self.0[nexti][nextj] {
                        Piece::O => {}
                        Piece::Wall => {}
                        Piece::Ground => {
                            self.0[nexti][nextj] = Piece::O;
                            self.0[i][j] = Piece::Ground;
                            changed = true;
                        }
                    }
                }
            }
        }
        changed
    }
    // I'm brainless now
    fn count_north_load(&self) -> u128 {
        let mut sum = 0u128;
        for j in 0..self.width() {
            for i in 0..self.height() {
                if self.0[i][j] == Piece::O {
                    sum += (self.height() - i) as u128;
                }
            }
        }
        sum
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Map(self
            .0
            .iter()
            .map(|row| row.iter().map(|p| p.clone()).collect_vec())
            .collect_vec())
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        if self.height() != other.height() || self.width() != other.width() {
            false
        } else {
            (0..self.height()).all(|i| (0..self.width()).all(|j| self.0[i][j] == other.0[i][j]))
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for piece in row {
                write!(f, "{}", piece);
            }
            writeln!(f, "");
        }
        Ok(())
    }
}

fn q1(input: &str) -> Result<u128, String> {
    let mut map: Map = input.parse()?;
    while map.tilt(&Direction::Up) {}
    print!("{}", map);
    Ok(map.count_north_load())
}

// Up 1 5 9
// left 2 6 10
// down 3 7 11
// right 4 8 12

fn q2(input: &str) -> Result<u128, String> {
    let mut map: Map = input.parse()?;
    let mut last_cycle = vec![];
    for i in 0..1_000_000_000 {
        while map.tilt(&Direction::Up) {}
        if i == 0 {
            println!("AFTER UP:\n{}", map)
        }
        while map.tilt(&Direction::Left) {}
        if i == 0 {
            println!("AFTER LEFT:\n{}", map)
        }
        while map.tilt(&Direction::Down) {}
        if i == 0 {
            println!("AFTER DOWN:\n{}", map)
        }
        while map.tilt(&Direction::Right) {}
        if let Some(pos) = last_cycle.iter().position(|lr: &Map| lr == &map) {
            let last_identical = pos;
            println!(
                " Current map:\n{}\nCurrent cycle {}. Last identical at {}",
                map,
                i + 1,
                last_identical + 1,
            );
            let cycle_len = i - last_identical;
            println!("Cycle len: {}", cycle_len);
            let target_mod = 1_000_000_000 % cycle_len;
            let (_, final_map) = last_cycle
                .iter()
                .skip(last_identical)
                .enumerate()
                .filter(|(pos, lr)| {
                    let my_mod = (last_identical + pos + 1) % cycle_len;
                    println!(
                        "Cycle number {} (mod {} = {}): load {}",
                        last_identical + pos + 1,
                        cycle_len,
                        my_mod,
                        lr.count_north_load()
                    );
                    my_mod == target_mod
                })
                .nth(0)
                .unwrap();
            println!("For 1_000_000_000 mod {} = {}", cycle_len, target_mod,);
            return Ok(final_map.count_north_load());
        } else {
            println!("Current cycle at {}\n{}", i + 1, map);
            last_cycle.push(map.clone());
        }
        if (i + 1 % 10_000_000 == 0) {
            println!("{}%", i / 10_000_000);
        }
    }
    Ok(map.count_north_load())
}

// 40 - 280
