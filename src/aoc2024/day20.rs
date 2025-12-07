use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

pub struct Day20;

impl Day for Day20 {
    fn question(&self, input: &str, question: Question) {
        let map: Map = input.parse().unwrap();
        let optimal = map.solve_maze().unwrap();
        let mut improvements = HashMap::new();
        let tot = (map.height() - 1) * (map.width() - 1);
        let mut i = 0;
        for y in 1..map.height() - 1 {
            for x in 1..map.width() - 1 {
                if let Some(testmap) = map.with_cheat(vec![Coord { x, y }]) {
                    let soln = testmap.solve_maze();
                    if let Some(soln) = soln {
                        let impr = if ((optimal - soln) > 100) {
                            100
                        } else {
                            optimal - soln
                        };
                        improvements
                            .entry(impr)
                            .and_modify(|mut n| *n += 1)
                            .or_insert(1usize);
                    }
                }
                i += 1;
                println!("{}/{}", i, tot);
            }
        }

        /*
                //horiz
        for y in 1..map.height() - 1 {
            for x in 1..map.width() - 2 {
                if let Some(testmap) = map.with_cheat(vec![Coord { x, y }, Coord { x: x + 1, y }]) {
                    let soln = testmap.solve_maze();
                    if let Some(soln) = soln {
                        let impr = if ((optimal - soln) > 100) {
                            100
                        } else {
                            optimal - soln
                        };
                        improvements
                            .entry(impr)
                            .and_modify(|mut n| *n += 1)
                            .or_insert(1usize);
                    }
                }
            }
        }
        //vertic
        for y in 1..map.height() - 2 {
            for x in 1..map.width() - 1 {
                if let Some(testmap) =
                    map.with_cheat(vec![Coord { x, y }, Coord { x: x, y: y + 1 }])
                {
                    let soln = testmap.solve_maze();
                    if let Some(soln) = soln {
                        let impr = if ((optimal - soln) > 100) {
                            100
                        } else {
                            optimal - soln
                        };
                        improvements
                            .entry(impr)
                            .and_modify(|mut n| *n += 1)
                            .or_insert(1usize);
                    }
                }
            }
        }

         */
        for k in improvements.keys().sorted() {
            println!("{} improvement: {}", k, improvements.get(k).unwrap());
        }
    }

    fn test_data(&self) -> String {
        "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
            .to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pixel {
    Wall,
    Corridor,
    End,
    Start,
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
                Pixel::Wall => '#',
                Pixel::Corridor => '.',
                Pixel::End => 'E',
                Pixel::Start => 'S',
            }
        )
    }
}

impl From<char> for Pixel {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Pixel::Wall,
            '.' => Pixel::Corridor,
            'E' => Pixel::End,
            'S' => Pixel::Start,
            _ => panic!("illegal char {}", ch),
        }
    }
}

struct Map(Vec<Vec<Pixel>>);

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|row| row.chars().map(|ch| Pixel::from(ch)).collect_vec())
            .collect_vec()))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Map {
    fn find_unique_pixel(&self, find_what: Pixel) -> Coord {
        for (coord, pixel) in self.iter() {
            if pixel == find_what {
                return coord;
            }
        }
        panic!("nonsense")
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }
    fn solve_maze(&self) -> Option<usize> {
        let mut bests_map = vec![vec![None; self.height()]; self.width()];
        let end = self.find_unique_pixel(Pixel::End);
        let mut work_queue = VecDeque::from([(end, 0)]);
        while let Some((coord, dist)) = work_queue.pop_front() {
            bests_map[coord.x][coord.y] = Some(dist);
            for neighbor in [
                Coord {
                    x: coord.x - 1,
                    y: coord.y,
                },
                Coord {
                    x: coord.x + 1,
                    y: coord.y,
                },
                Coord {
                    x: coord.x,
                    y: coord.y - 1,
                },
                Coord {
                    x: coord.x,
                    y: coord.y + 1,
                },
            ] {
                if self[neighbor] != Pixel::Wall
                    && (bests_map[neighbor.x][neighbor.y].is_none()
                        || bests_map[neighbor.x][neighbor.y].unwrap() > dist + 1)
                {
                    work_queue.push_front((neighbor, dist + 1));
                }
            }
        }
        let start = self.find_unique_pixel(Pixel::Start);
        bests_map[start.x][start.y].clone()
    }
    fn iter(&self) -> MapAllPixelsIterator {
        MapAllPixelsIterator {
            map: self,
            current: Coord { x: 0, y: 0 },
        }
    }
    fn with_cheat(&self, coords: Vec<Coord>) -> Option<Map> {
        if coords
            .iter()
            .any(|c| self[*c] == Pixel::End || self[*c] == Pixel::Start)
        {
            None
        } else if coords.iter().all(|c| self[*c] == Pixel::Corridor) {
            None
        } else {
            let mut newmap = Map(self.0.clone());
            for coord in coords {
                newmap[coord] = Pixel::Corridor;
            }
            Some(newmap)
        }
    }
}

impl Index<Coord> for Map {
    type Output = Pixel;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl IndexMut<Coord> for Map {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}

struct MapAllPixelsIterator<'a> {
    map: &'a Map,
    current: Coord,
}

impl Iterator for MapAllPixelsIterator<'_> {
    type Item = (Coord, Pixel);

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.current.clone();
        if curr.y == self.map.height() {
            None
        } else {
            if curr.x + 1 == self.map.width() {
                self.current = Coord {
                    x: 0,
                    y: curr.y + 1,
                }
            } else {
                self.current = Coord {
                    x: curr.x + 1,
                    y: curr.y,
                }
            }

            Some((
                Coord {
                    x: curr.x.clone(),
                    y: curr.y.clone(),
                },
                self.map[curr],
            ))
        }
    }
}
