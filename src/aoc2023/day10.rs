use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day10;

impl Day for Day10 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res)
    }

    fn test_data(&self) -> String {
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            .to_string()
    }
}

// row, column
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coord(pub usize, pub usize);
impl Coord {
    fn x(&self) -> usize {
        // column
        self.1
    }
    fn y(&self) -> usize {
        // row
        self.0
    }
    fn to_left(&self) -> Self {
        Coord(self.0, self.1 - 1)
    }
    fn to_right(&self) -> Self {
        Coord(self.0, self.1 + 1)
    }
    fn to_up(&self) -> Self {
        Coord(self.0 - 1, self.1)
    }
    fn to_down(&self) -> Self {
        Coord(self.0 + 1, self.1)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x={} y={}", self.x(), self.y())
    }
}

// I know this is not needed, but therapeutic
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Ground,
    Starter,
    Horizontal,
    Vertical,
    L,
    F,
    J,
    Seven,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Tile::*;
        match value {
            '.' => Ok(Ground),
            'S' => Ok(Starter),
            '-' => Ok(Horizontal),
            '|' => Ok(Vertical),
            'L' => Ok(L),
            'F' => Ok(F),
            'J' => Ok(J),
            '7' => Ok(Seven),
            _ => Err(format!("illegal tile character: {}", value)),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ground => write!(f, "."),
            Tile::Starter => write!(f, "S"),
            Tile::Horizontal => write!(f, "-"),
            Tile::Vertical => write!(f, "|"),
            Tile::L => write!(f, "L"),
            Tile::F => write!(f, "F"),
            Tile::J => write!(f, "J"),
            Tile::Seven => write!(f, "7"),
        }
    }
}

impl Tile {
    fn next_coords(&self, coord: &Coord) -> Result<(Coord, Coord), String> {
        match self {
            Tile::Ground => Err(format!("we hit the ground! {}", coord)),
            Tile::Starter => Err(format!("we got back to starter")),
            Tile::Horizontal => Ok((coord.to_left(), coord.to_right())),
            Tile::Vertical => Ok((coord.to_up(), coord.to_down())),
            Tile::L => Ok((coord.to_up(), coord.to_right())),
            Tile::F => Ok((coord.to_down(), coord.to_right())),
            Tile::J => Ok((coord.to_left(), coord.to_up())),
            Tile::Seven => Ok((coord.to_left(), coord.to_down())),
        }
    }
}

pub struct Map(Vec<Vec<Tile>>);

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Result<Vec<Vec<Tile>>, String> = s
            .lines()
            .map(|line| line.chars().map(|ch| Tile::try_from(ch)).collect())
            .collect();
        rows.map(|v| Map(v))
    }
}

impl From<Vec<Vec<Tile>>> for Map {
    fn from(x: Vec<Vec<Tile>>) -> Self {
        Map(x)
    }
}

#[derive(Eq, PartialEq)]
enum WalkThruState {
    Outside,
    Inside,
    InsideAfterL,
    OutsideAfterL,
    InsideAfterF,
    OutsideAfterF,
}

impl Map {
    fn find_starter(&self) -> Coord {
        let fnd = self
            .0
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find(|(j, col)| **col == Tile::Starter)
                    .map(|(j, _)| (i, j))
            })
            .filter(|row| row.is_some())
            .next();
        let (x, y) = fnd.unwrap().unwrap();
        Coord(x, y)
    }
    fn get(&self, coord: &Coord) -> Option<Tile> {
        if coord.0 < 0 || coord.1 < 0 {
            None
        } else if coord.0 >= self.height() || coord.1 >= self.width() {
            None
        } else {
            Some(self.0[coord.0][coord.1])
        }
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }

    pub fn pipes(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|tile| tile != &&Tile::Ground).count())
            .sum::<usize>()
    }

    pub fn draw_pipe_map(&self, pipe_map: &HashMap<Coord, bool>) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                if pipe_map.contains_key(&Coord(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell);
            }
            writeln!(f);
        }
        Ok(())
    }
}

// done with functional on day 10?!
fn q(input: &str, question: Question) -> Result<(usize, usize), String> {
    let map: Map = input.parse()?;
    let starter = map.find_starter();
    let neighbors = [
        if starter.1 == 0 {
            None
        } else {
            Some(starter.to_left())
        },
        if starter.0 == 0 {
            None
        } else {
            Some(starter.to_up())
        },
        if starter.1 == map.width() - 1 {
            None
        } else {
            Some(starter.to_right())
        },
        if starter.0 == map.height() - 1 {
            None
        } else {
            Some(starter.to_down())
        },
    ];
    let neighbors = neighbors.into_iter().filter_map(|x| x.as_ref());
    let mut pipe_map = HashMap::new(); // could be set but maybe I want non-bool
    let (rt1, rt2) = neighbors
        .into_iter()
        .filter(|neighbor| {
            if let Some(tile) = map.get(neighbor) {
                let neighbors_nexts = tile.next_coords(neighbor);
                if neighbors_nexts.is_err() {
                    false
                } else {
                    let (n1, n2) = neighbors_nexts.unwrap();
                    n1 == starter || n2 == starter
                }
            } else {
                false
            }
        })
        .collect_tuple()
        .ok_or("Not exactly 2 legit moves from starter")?;
    let mut rt1 = *rt1;
    let mut rt2 = *rt2;
    let mut last_rt1 = starter;
    let mut last_rt2 = starter;
    pipe_map.insert(starter, true);
    pipe_map.insert(rt1, true);
    pipe_map.insert(rt2, true);
    let mut steps = 1;
    let steps = loop {
        println!("We at {} {}", rt1, rt2);
        steps += 1;
        let tile1 = map
            .get(&rt1)
            .ok_or(format!("We ran out of bounds at {}", rt1))?;
        let nexts = tile1.next_coords(&rt1)?;
        let tmp = rt1;
        rt1 = if nexts.0 == last_rt1 {
            nexts.1
        } else {
            nexts.0
        };
        last_rt1 = tmp;
        if rt1 == rt2 {
            break steps - 1;
        }
        pipe_map.insert(rt1, true);
        let tile2 = map
            .get(&rt2)
            .ok_or(format!("We ran out of bounds at {}", rt2))?;
        let nexts = tile2.next_coords(&rt2)?;
        let tmp = rt2;
        rt2 = if nexts.0 == last_rt2 {
            nexts.1
        } else {
            nexts.0
        };
        pipe_map.insert(rt2, true);
        last_rt2 = tmp;
        if rt1 == rt2 {
            // return steps;
            break steps;
        }
    };
    let inside = calculate_insides(&map, &pipe_map);
    Ok((steps, inside))
}

pub fn calculate_insides(map: &Map, pipe_map: &HashMap<Coord, bool>) -> usize {
    let mut inside = 0usize;
    for i in 0..map.height() {
        let mut state = WalkThruState::Outside;
        for j in 0..map.width() {
            let tile = map.get(&Coord(i, j)).unwrap();
            let in_pipe = pipe_map.get(&Coord(i, j)).is_some();
            if state == WalkThruState::InsideAfterF {
                // must be in pipe
                state = match tile {
                    Tile::Starter | Tile::Horizontal => WalkThruState::InsideAfterF, // st?
                    Tile::Seven => WalkThruState::Inside,
                    Tile::J => WalkThruState::Outside,
                    _ => panic!("no"),
                };
                print!("p");
            } else if state == WalkThruState::OutsideAfterF {
                state = match tile {
                    Tile::Starter | Tile::Horizontal => WalkThruState::OutsideAfterF, // st?
                    Tile::Seven => WalkThruState::Outside,
                    Tile::J => WalkThruState::Inside,
                    _ => panic!("no"),
                };
                print!("π");
            } else if state == WalkThruState::InsideAfterL {
                // must be in pipe
                state = match tile {
                    Tile::Starter | Tile::Horizontal => WalkThruState::InsideAfterL, // st?
                    Tile::Seven => WalkThruState::Outside,
                    Tile::J => WalkThruState::Inside,
                    _ => panic!("no"),
                };
                print!("l");
            } else if state == WalkThruState::OutsideAfterL {
                state = match tile {
                    Tile::Starter | Tile::Horizontal => WalkThruState::OutsideAfterL, // st?
                    Tile::Seven => WalkThruState::Inside,
                    Tile::J => WalkThruState::Outside,
                    _ => panic!("no"),
                };
                print!("q");
            } else if state == WalkThruState::Inside && !in_pipe {
                inside += 1; // the only case
                print!("*");
            } else if state == WalkThruState::Inside {
                // we are in_pipe
                print!("P");
                state = match tile {
                    Tile::Ground => panic!(
                        "this isn't the world I want to live in. {} {} {} {:?}",
                        i,
                        j,
                        tile,
                        pipe_map.get(&Coord(i, j))
                    ),
                    Tile::Starter | Tile::Vertical => WalkThruState::Outside, // starter is | in input; F in first test,7 in second test...
                    Tile::Horizontal => WalkThruState::Inside,
                    Tile::L => WalkThruState::InsideAfterL,
                    Tile::F => WalkThruState::InsideAfterF,
                    Tile::J | Tile::Seven => WalkThruState::Outside,
                };
            } else if in_pipe && state == WalkThruState::Outside {
                // we're outside
                print!("P");
                state = match tile {
                    Tile::Ground => panic!("life has too much pain anyway :("),
                    Tile::Horizontal => WalkThruState::Outside,
                    Tile::F => WalkThruState::OutsideAfterF,
                    Tile::L => WalkThruState::OutsideAfterL,
                    Tile::Vertical => WalkThruState::Inside,
                    Tile::Starter => WalkThruState::Inside,
                    Tile::J | Tile::Seven => WalkThruState::Outside, // I dont think this is possible
                };
            } else {
                // else we're outside and not in pipe so irrelevant
                print!(".");
            }
        }
        println!();
    }
    inside
}
