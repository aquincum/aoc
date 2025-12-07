use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

pub struct Day18;

impl Day for Day18 {
    fn question(&self, input: &str, question: Question) {
        let is_test = input.chars().nth(0) == Some('5');
        let (width, height) = match is_test {
            true => (7, 7),
            false => (71, 71),
        };
        let limit = if is_test { 12 } else { 1024 };
        let all_blocks: Result<Blockfall, String> = input.parse();
        if all_blocks.is_err() {
            println!("Can't read all_blocks: {}", all_blocks.err().unwrap());
            return;
        }
        let all_blocks = all_blocks.unwrap();
        let all_blocks = all_blocks.with_dimensions(width, height);
        let map = all_blocks.create_map(limit);
        println!("Map:\n{}\n===", map);
        let res = map.solve_1();
        println!("Q1: {}", res.unwrap());
        let mut begin = limit + 1;
        let mut end = all_blocks.blocks.len();
        while begin < end {
            let to_test = (begin + end) / 2;
            let test_map = all_blocks.create_map(to_test);
            let res = test_map.solve_1();
            if res.is_none() {
                end = to_test;
                println!(
                    "Q2: with {} no solution || running {}-{}",
                    to_test, begin, end
                );
            } else {
                begin = to_test + 1;
                println!(
                    "Q2: with {} yes solution: {} || running {}-{}",
                    to_test,
                    res.unwrap(),
                    begin,
                    end
                );
                if begin == end {}
            }
        }
        println!(
            "Crucial brick: {},{}",
            all_blocks.blocks[begin - 1].0,
            all_blocks.blocks[begin - 1].1,
        )
    }

    fn test_data(&self) -> String {
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
        .to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pixel {
    Block,
    Empty,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::Block => '#',
                Pixel::Empty => '.',
            }
        )
    }
}

impl From<char> for Pixel {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Pixel::Block,
            '.' => Pixel::Empty,
            _ => panic!("illegal char {}", ch),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

struct Blockfall {
    blocks: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl FromStr for Blockfall {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Result<Vec<(usize, usize)>, String> = s
            .lines()
            .map(|line| {
                let (x, y) = line
                    .split(",")
                    .collect_tuple()
                    .ok_or("no pair".to_string())?;
                let x = x.parse().map_err(|e: ParseIntError| e.to_string())?;
                let y = y.parse().map_err(|e: ParseIntError| e.to_string())?;
                Ok((x, y))
            })
            .collect();
        Ok(Self {
            blocks: blocks?,
            width: 0,
            height: 0,
        })
    }
}

impl Blockfall {
    fn with_dimensions(self, width: usize, height: usize) -> Blockfall {
        Blockfall {
            blocks: self.blocks,
            width,
            height,
        }
    }
    fn create_map(&self, limit: usize) -> Map {
        let blocks: HashSet<(usize, usize)> = self.blocks.iter().take(limit).map(|x| *x).collect();
        println!("LAST {:?}", self.blocks.iter().take(limit).last());
        let map = Map((0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| {
                        if blocks.contains(&(x, y)) {
                            Pixel::Block
                        } else {
                            Pixel::Empty
                        }
                    })
                    .collect()
            })
            .collect());
        map
    }
}

struct Map(Vec<Vec<Pixel>>);
struct VisitedMap(Vec<Vec<Option<u128>>>);

impl Index<Coord> for Map {
    type Output = Pixel;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl Index<Coord> for VisitedMap {
    type Output = Option<u128>;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl IndexMut<Coord> for VisitedMap {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}

impl VisitedMap {
    fn new(width: usize, height: usize) -> Self {
        VisitedMap(vec![vec![None; width]; height])
    }
}

impl Map {
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }
    fn get_neighbor(&self, coord: Coord, direction: Direction) -> Option<Coord> {
        match direction {
            Direction::Up if coord.y == 0 => None,
            Direction::Up => Some(Coord {
                x: coord.x,
                y: coord.y - 1,
            }),
            Direction::Down if coord.y == self.height() - 1 => None,
            Direction::Down => Some(Coord {
                x: coord.x,
                y: coord.y + 1,
            }),
            Direction::Left if coord.x == 0 => None,
            Direction::Left => Some(Coord {
                x: coord.x - 1,
                y: coord.y,
            }),
            Direction::Right if coord.x == self.width() - 1 => None,
            Direction::Right => Some(Coord {
                x: coord.x + 1,
                y: coord.y,
            }),
        }
    }
    fn solve_1(&self) -> Option<u128> {
        let starter = Coord {
            x: self.width() - 1,
            y: self.height() - 1,
        };
        let mut work_queue = VecDeque::from([(starter, 0u128)]);
        let mut visited_map = VisitedMap::new(self.width(), self.height());
        let mut i = 0;
        while let Some((coord, score)) = work_queue.pop_front() {
            visited_map[coord] = Some(score);
            for neighbor in [
                self.get_neighbor(coord, Direction::Left),
                self.get_neighbor(coord, Direction::Right),
                self.get_neighbor(coord, Direction::Up),
                self.get_neighbor(coord, Direction::Down),
            ] {
                i += -1;
                if let Some(neighbor) = neighbor {
                    // println!(
                    //     "testing {} {} {} {:?}",
                    //     neighbor.x, neighbor.y, self[neighbor], visited_map[neighbor]
                    // );
                    if self[neighbor] == Pixel::Empty
                        && (visited_map[neighbor].is_none()
                            || visited_map[neighbor].unwrap() > score + 1)
                    {
                        work_queue.push_front((neighbor, score + 1));
                    }
                }
            }
        }
        visited_map[Coord { x: 0, y: 0 }]
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.height() {
            for y in 0..self.width() {
                let p = self[Coord { x, y }];
                write!(f, "{}", p);
            }
            writeln!(f);
        }
        Ok(())
    }
}
