use crate::aoc2023::day10::{calculate_insides, Coord, Map, Tile};
use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day18;

impl Day for Day18 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            .to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    L,
    R,
    U,
    D,
}

type Color = String;

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::L),
            'R' => Ok(Direction::R),
            'U' => Ok(Direction::U),
            'D' => Ok(Direction::D),
            _ => Err(()),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            Err(())
        } else {
            TryFrom::try_from(s.chars().nth(0).unwrap())
        }
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::L => "L".to_string(),
            Direction::R => "R".to_string(),
            Direction::U => "U".to_string(),
            Direction::D => "D".to_string(),
        }
    }
}

#[derive(Clone)]
struct Move {
    direction: Direction,
    color: Color,
    moves: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, moves, color) = s.split(" ").collect_tuple().ok_or(())?;
        let direction = direction.parse::<Direction>().map_err(|_| ())?;
        let moves = moves.parse::<usize>().map_err(|_| ())?;
        let color = color
            .trim_start_matches("(#")
            .trim_start_matches(")")
            .to_string();
        Ok(Move {
            direction,
            color,
            moves,
        })
    }
}

#[derive(Debug)]
struct Dimensions {
    width: usize,
    height: usize,
    starter_x: usize,
    starter_y: usize,
}
fn find_dims(moves: &Vec<Move>) -> Dimensions {
    let mut max_x = 0isize;
    let mut max_y = 0isize;
    let mut min_x = 0isize;
    let mut min_y = 0isize;
    let mut x = 0isize;
    let mut y = 0isize;
    for mv in moves.iter() {
        match mv.direction {
            Direction::L => {
                x -= mv.moves as isize;
            }
            Direction::R => {
                x += mv.moves as isize;
            }
            Direction::U => {
                y -= mv.moves as isize;
            }
            Direction::D => {
                y += mv.moves as isize;
            }
        }
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }
    Dimensions {
        width: (max_x - min_x + 1) as usize,
        height: (max_y - min_y + 1) as usize,
        starter_x: (-min_x) as usize,
        starter_y: (-min_y) as usize,
    }
}

fn build_maps(moves: &Vec<Move>) -> (Map, HashMap<Coord, bool>) {
    let dims = find_dims(moves);
    let mut map = (0..dims.height)
        .map(|y| (0..dims.width).map(|x| Tile::Ground).collect_vec())
        .collect_vec();

    let mut x = dims.starter_x;
    let mut y = dims.starter_y;
    let mut last_dir = Direction::U;
    let mut pipe_map = HashMap::new();
    for mv in moves.iter() {
        let pipe_tile = calculate_tile(&last_dir, mv);
        map[y][x] = pipe_tile;
        pipe_map.insert(Coord(y, x), true);

        for i in 0..mv.moves {
            match mv.direction {
                Direction::L => {
                    x -= 1;
                    map[y][x] = Tile::Horizontal;
                }
                Direction::R => {
                    x += 1;
                    map[y][x] = Tile::Horizontal;
                }
                Direction::U => {
                    y -= 1;
                    map[y][x] = Tile::Vertical;
                }
                Direction::D => {
                    y += 1;
                    map[y][x] = Tile::Vertical;
                }
            }
            pipe_map.insert(Coord(y, x), true);
        }
        last_dir = mv.direction;
    }
    // finally
    map[dims.starter_y][dims.starter_x] = calculate_tile(&last_dir, &moves[0]);

    (From::from(map), pipe_map)
}

fn calculate_tile(last_dir: &Direction, mv: &Move) -> Tile {
    match (last_dir, mv.direction) {
        (Direction::D, Direction::D) => Tile::Vertical,
        (Direction::D, Direction::L) => Tile::J,
        (Direction::D, Direction::R) => Tile::L,
        (Direction::U, Direction::U) => Tile::Vertical,
        (Direction::U, Direction::L) => Tile::Seven,
        (Direction::U, Direction::R) => Tile::F,
        (Direction::R, Direction::R) => Tile::Horizontal,
        (Direction::R, Direction::U) => Tile::J,
        (Direction::R, Direction::D) => Tile::Seven,
        (Direction::L, Direction::L) => Tile::Horizontal,
        (Direction::L, Direction::U) => Tile::L,
        (Direction::L, Direction::D) => Tile::F,

        (x, y) => panic!("incorrect combo {} {}", x.to_string(), y.to_string()),
    }
}

fn q1(input: &str) -> Result<u128, String> {
    let moves = input
        .lines()
        .map(|l| l.parse::<Move>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "move coll error".to_string())?;
    let (map, pipe_map) = build_maps(&moves);
    println!("MAP:\n{}", map);
    map.draw_pipe_map(&pipe_map);
    let insides = calculate_insides(&map, &pipe_map);
    let outsides = map.pipes();
    Ok(insides as u128 + outsides as u128)
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Span {
    from: usize,
    to: usize,
}

impl Span {
    fn length(&self) -> usize {
        self.to - self.from + 1
    }
}
fn q2(input: &str) -> Result<u128, String> {
    let moves = input
        .lines()
        .map(|l| l.parse::<Move>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "move coll error".to_string())?;
    let moves = moves
        .into_iter()
        .map(|mv| {
            let dist = usize::from_str_radix(&mv.color[0..5], 16)
                .map_err(|e: ParseIntError| e.to_string());
            let dir = match mv.color.chars().nth(5).ok_or("No fifth".to_string())? {
                '0' => Ok(Direction::R),
                '1' => Ok(Direction::D),
                '2' => Ok(Direction::U),
                '3' => Ok(Direction::L),
                _ => Err("Invalid direction".to_string()),
            };
            dist.and_then(|d| {
                Ok(Move {
                    direction: dir?,
                    color: "".to_string(),
                    moves: d,
                })
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let dims = find_dims(&moves);
    println!("Dims: {:?}", dims);
    let mut horizontals = HashMap::new();
    let mut verticals = HashMap::new();
    let mut x = dims.starter_x;
    let mut y = dims.starter_y;
    for mv in moves.iter() {
        if mv.direction == Direction::L || mv.direction == Direction::R {
            let endx = if mv.direction == Direction::L {
                x - mv.moves
            } else {
                x + mv.moves
            };
            let span = Span {
                from: x.min(endx),
                to: x.max(endx),
            };
            horizontals
                .entry(y)
                .and_modify(|v: &mut Vec<Span>| v.push(span))
                .or_insert(vec![span]);
        } else {
            let endy = if mv.direction == Direction::U {
                y - mv.moves
            } else {
                y + mv.moves
            };
            let span = Span {
                from: y.min(endy),
                to: y.max(endy),
            };
            verticals
                .entry(x)
                .and_modify(|v: &mut Vec<Span>| v.push(span))
                .or_insert(vec![span]);
        }
    }

    let mut current_in_spans: Vec<Span> = vec![];
    let mut sum = 0u128;
    for row_idx in 0..dims.height {
        for span in horizontals.get(&row_idx).unwrap_or(&vec![]) {
            sum += span.length() as u128;
        }
        for span in current_in_spans.iter() {
            sum += span.length() as u128; // todo ??
        }
        // verticals.iter().filter(|(col_idx,v)|)
    }

    Ok(sum)
}
