use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::Index;
use std::str::FromStr;

pub struct Day21;

impl Day for Day21 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
            .to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Field {
    Plot,
    Rock,
    Starter,
}

impl From<char> for Field {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Field::Plot,
            '#' => Field::Rock,
            _ => Field::Starter,
        }
    }
}

struct Area(Vec<Vec<Field>>);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Coord(usize, usize);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct InfCoord(isize, isize);

type FieldMap = HashSet<Coord>;

fn read_map(input: &str) -> Area {
    let area = input
        .lines()
        .map(|line| line.chars().map(|ch| Field::from(ch)).collect_vec())
        .collect_vec();
    Area(area)
}

// maybe unused?
fn rock_map(area: &Area) -> FieldMap {
    (0..area.len())
        .map(|row| {
            (0..area[row].len()).filter_map(move |col| match area[row][col] {
                Field::Plot | Field::Starter => None,
                Field::Rock => Some(Coord(col, row)),
            })
        })
        .flatten()
        .collect()
}

// maybe unused?
fn plot_map(area: &Area) -> FieldMap {
    (0..area.len())
        .map(|row| {
            (0..area[row].len()).filter_map(move |col| match area[row][col] {
                Field::Plot | Field::Starter => Some(Coord(col, row)),
                Field::Rock => None,
            })
        })
        .flatten()
        .collect()
}

fn find_starter(area: &Area) -> Coord {
    for row in 0..area.len() {
        for col in 0..area[row].len() {
            if area[row][col] == Field::Starter {
                return Coord(col, row);
            }
        }
    }
    panic!("No starter found")
}

impl Area {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn in_bounds_coord(&self, coord: (isize, isize)) -> Option<Coord> {
        if coord.0 < 0 || coord.1 < 0 {
            None
        } else if coord.1 as usize >= self.len() || coord.0 as usize >= self[coord.1 as usize].len()
        {
            None
        } else {
            Some(Coord(coord.0 as usize, coord.1 as usize))
        }
    }
    fn valid_neighbors(&self, coord: &Coord) -> Vec<Coord> {
        [
            (coord.0 as isize - 1, coord.1 as isize),
            (coord.0 as isize + 1, coord.1 as isize),
            (coord.0 as isize, coord.1 as isize - 1),
            (coord.0 as isize, coord.1 as isize + 1),
        ]
        .into_iter()
        .filter_map(|icoords| self.in_bounds_coord(*icoords))
        .filter(|c| self[c] != Field::Rock)
        .collect_vec()
    }
    fn valid_neighbors_infinity(&self, coord: &InfCoord) -> Vec<InfCoord> {
        [
            InfCoord(coord.0 - 1, coord.1),
            InfCoord(coord.0 + 1, coord.1),
            InfCoord(coord.0, coord.1 - 1),
            InfCoord(coord.0, coord.1 + 1),
        ]
        .into_iter()
        .filter(|c| self[*c] != Field::Rock)
        .map(|c| *c)
        .collect_vec()
    }
}

fn print_area(area: &Area, currs: &HashSet<Coord>) {
    for row in 0..area.0.len() {
        for col in 0..area.0[row].len() {
            if currs.contains(&Coord(col, row)) {
                print!("O");
            } else {
                print!(
                    "{}",
                    match area.0[row][col] {
                        Field::Plot => ".",
                        Field::Rock => "#",
                        Field::Starter => "S",
                    }
                );
            }
        }
        println!();
    }
}

impl Index<usize> for Area {
    type Output = Vec<Field>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<&Coord> for Area {
    type Output = Field;

    fn index(&self, index: &Coord) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

// https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation-
fn modulo(a: isize, b: usize) -> usize {
    let b = b as isize;
    (((a % b) + b) % b) as usize
}

impl Index<&InfCoord> for Area {
    type Output = Field;

    fn index(&self, index: &InfCoord) -> &Self::Output {
        let i1 = modulo(index.1, self.0.len());
        let i0 = modulo(index.0, self.0[0].len());
        &self.0[i1][i0]
    }
}

fn q1(input: &str) -> Result<u128, String> {
    let area = read_map(input);
    // let rocks = rock_map(&area);
    let starter = find_starter(&area);
    const STEPS: usize = 64;
    let finals = (0..STEPS).fold(HashSet::from([starter]), |valids, i| {
        // println!("BEFORE STEP {}", i);
        // print_area(&area, &valids);
        valids
            .iter()
            .map(|coord| area.valid_neighbors(coord))
            .flatten()
            .collect()
    });
    // println!("FINAL");
    // print_area(&area, &finals);
    println!("{:?}", finals);
    Ok(finals.len() as u128)
}

fn q2(input: &str) -> Result<u128, String> {
    // we're only going to debug here
    let area = read_map(input);
    let starter = find_starter(&area);
    let starter = InfCoord(starter.0 as isize, starter.1 as isize);
    let side = area.len();
    let steps: usize = 10 * side + 1;
    let mut last = 0;
    let mut lastdiff = 0isize;
    let mut lastdiffdiff = 0isize;
    let finals = (0..steps).fold(HashSet::from([starter]), |valids, i| {
        if i % side == 65 {
            let diff = valids.len() - last;
            let diff2 = lastdiff - diff as isize;
            println!(
                "AFTER STEP {} number of valids: {} [diff={}] [diff2={}]",
                i,
                valids.len(),
                diff,
                diff2,
            );
            last = valids.len();
            lastdiff = diff as isize;
        }
        valids
            .iter()
            .map(|coord| area.valid_neighbors_infinity(coord))
            .flatten()
            .collect()
    });
    // println!("FINAL");
    // print_area(&area, &finals);
    // println!("{:?}", finals);
    Ok(finals.len() as u128)
}
