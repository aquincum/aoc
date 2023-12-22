use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day16;

impl Day for Day16 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
            .to_string()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Piece {
    Ground,
    SplitVertically,
    SplitHorizontally,
    Slash,
    Backslash,
}

impl From<char> for Piece {
    fn from(ch: char) -> Self {
        match ch {
            '\\' => Piece::Backslash,
            '/' => Piece::Slash,
            '|' => Piece::SplitVertically,
            '-' => Piece::SplitHorizontally,
            _ => Piece::Ground,
        }
    }
}

struct Map(Vec<Vec<Piece>>);
type HistoryMap = Vec<Vec<HashSet<Direction>>>;

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|line| line.chars().map(|ch| Piece::from(ch)).collect_vec())
            .collect_vec()))
    }
}

impl Map {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
    fn next_step(&self, ray: &Ray) -> Option<(usize, usize)> {
        match ray.direction {
            Direction::Right => {
                if ray.location.0 == self.width() - 1 {
                    None
                } else {
                    Some((ray.location.0 + 1, ray.location.1))
                }
            }
            Direction::Left => {
                if ray.location.0 == 0 {
                    None
                } else {
                    Some((ray.location.0 - 1, ray.location.1))
                }
            }
            Direction::Up => {
                if ray.location.1 == 0 {
                    None
                } else {
                    Some((ray.location.0, ray.location.1 - 1))
                }
            }
            Direction::Down => {
                if ray.location.1 == self.height() - 1 {
                    None
                } else {
                    Some((ray.location.0, ray.location.1 + 1))
                }
            }
        }
    }
}

// I won't recurse bc I'm fairly certain it would lead to stack overflow :(((

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Ray {
    direction: Direction,
    location: (usize, usize),
}

fn track_ray(map: &Map, energy_map: &mut HistoryMap, initial: Ray) -> Vec<Ray> {
    let mut alternatives = vec![];
    let mut ray = initial;
    loop {
        if energy_map[ray.location.1][ray.location.0].contains(&ray.direction) {
            break; // we have been here
        }
        energy_map[ray.location.1][ray.location.0].insert(ray.direction);
        match map.0[ray.location.1][ray.location.0] {
            Piece::Ground => {}
            Piece::SplitVertically => {
                if ray.direction == Direction::Left || ray.direction == Direction::Right {
                    ray.direction = Direction::Up;
                    alternatives.push(Ray {
                        location: ray.location,
                        direction: Direction::Down,
                    })
                }
            }
            Piece::SplitHorizontally => {
                if ray.direction == Direction::Up || ray.direction == Direction::Down {
                    ray.direction = Direction::Left;
                    alternatives.push(Ray {
                        location: ray.location,
                        direction: Direction::Right,
                    })
                }
            }
            Piece::Slash => {
                ray.direction = match ray.direction {
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                }
            }
            Piece::Backslash => {
                ray.direction = match ray.direction {
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                }
            }
        }
        let new_loc = map.next_step(&ray);
        if new_loc.is_none() {
            break;
        }
        ray.location = new_loc.unwrap();
    }
    alternatives
}

fn q1(input: &str) -> Result<u128, String> {
    let map: Map = input.parse()?;
    let mut energy_map = (0..map.height())
        .map(|_| {
            (0..map.width())
                .map(|_| HashSet::<Direction>::new())
                .collect_vec()
        })
        .collect_vec();
    let initial_ray = Ray {
        location: (0, 0),
        direction: Direction::Right,
    };
    Ok(run_rays(&map, &mut energy_map, initial_ray))
}

fn run_rays(
    map: &Map,
    mut energy_map: &mut Vec<Vec<HashSet<Direction>>>,
    initial_ray: Ray,
) -> u128 {
    let mut alternatives = track_ray(&map, &mut energy_map, initial_ray);
    let mut dones = HashSet::new();
    dones.insert(initial_ray);
    while let Some(current) = alternatives.pop() {
        dones.insert(current);
        let mut new_alts = track_ray(&map, &mut energy_map, current)
            .into_iter()
            .filter(|r| !dones.contains(r))
            .collect_vec();
        alternatives.append(&mut new_alts);
    }
    energy_map
        .iter()
        .map(|r| {
            r.iter()
                .map(|set| if set.is_empty() { 0 } else { 1 })
                .sum::<usize>()
        })
        .sum::<usize>() as u128
}

fn q2(input: &str) -> Result<u128, String> {
    let map: Map = input.parse()?;
    let lefts = (0..map.height())
        .map(|y| Ray {
            location: (0, y),
            direction: Direction::Right,
        })
        .collect_vec();
    let rights = (0..map.height())
        .map(|y| Ray {
            location: (map.width() - 1, y),
            direction: Direction::Left,
        })
        .collect_vec();
    let tops = (0..map.height())
        .map(|x| Ray {
            location: (x, 0),
            direction: Direction::Down,
        })
        .collect_vec();
    let bottoms = (0..map.height())
        .map(|x| Ray {
            location: (x, map.height() - 1),
            direction: Direction::Up,
        })
        .collect_vec();
    let all_inits = vec![lefts, rights, tops, bottoms].concat();
    let all_rays = all_inits.into_iter().map(|init| {
        let mut energy_map = (0..map.height())
            .map(|_| {
                (0..map.width())
                    .map(|_| HashSet::<Direction>::new())
                    .collect_vec()
            })
            .collect_vec();
        run_rays(&map, &mut energy_map, init)
    });
    all_rays.max().ok_or("no max?!".to_string())
}
