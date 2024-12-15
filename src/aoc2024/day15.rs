use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day15;

impl Day for Day15 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .to_string()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Pixel {
    Empty,
    Wall,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
}

impl From<char> for Pixel {
    fn from(ch: char) -> Self {
        use Pixel::*;
        match ch {
            '#' => Wall,
            '.' => Empty,
            '@' => Robot,
            'O' => Box,
            _ => panic!("Illegal char in map {}", ch),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::Empty => '.',
                Pixel::Wall => '#',
                Pixel::Box => 'O',
                Pixel::Robot => '@',
                Pixel::BoxLeft => '[',
                Pixel::BoxRight => ']',
            }
        )
    }
}

impl Pixel {
    fn double_pixel(self) -> Vec<Pixel> {
        use Pixel::*;
        match self {
            Wall => vec![Wall, Wall],
            Empty => vec![Empty, Empty],
            Robot => vec![Robot, Empty],
            Box => vec![BoxLeft, BoxRight],
            _ => panic!("Illegal char in map {:?}", self),
        }
    }
    fn moveable(&self) -> bool {
        *self == Pixel::Box
    }
}

struct Map {
    map: Vec<Vec<Pixel>>,
    robot: (usize, usize), // cache it
}

impl Map {
    fn read_map(s: &str, question: Question) -> Result<Self, String> {
        let map = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        let pix = Pixel::from(ch);
                        if question == Question::Second {
                            pix.double_pixel()
                        } else {
                            vec![pix]
                        }
                    })
                    .flatten()
                    .collect_vec()
            })
            .collect_vec();
        // let robot = map.iter().enumerate().find_position(|(_, row)| {
        //     row.iter()
        //         .enumerate()
        //         .find_position(|(j, p)| {
        //             println!("{} is a {}", j, **p);
        //             **p == Pixel::Robot
        //         })
        //         .is_some()
        // }); // this works perfectly for q1, but VERY WEIRDLY gives the old # for q2?/!

        let mut robot = None;
        for (i, row) in map.iter().enumerate() {
            for (j, pix) in row.iter().enumerate() {
                if *pix == Pixel::Robot {
                    robot = Some((j, i));
                    break;
                }
            }
            if robot.is_some() {
                break;
            }
        }

        println!("UMMM {:?}", robot);
        let robot = robot.ok_or("No robot?".to_string())?;
        // let (x, (y, _)) = robot;
        // let robot = (x, y);
        Ok(Map { map, robot })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Move {
    Up,
    Left,
    Right,
    Down,
}

impl From<char> for Move {
    fn from(ch: char) -> Self {
        use Move::*;
        match ch {
            '^' => Up,
            '<' => Left,
            '>' => Right,
            'v' => Down,
            _ => panic!("Illegal char in map {}", ch),
        }
    }
}

impl Move {
    fn transform(&self, (x, y): &(usize, usize)) -> Option<(usize, usize)> {
        match self {
            Move::Up => {
                if *y == 0 {
                    None
                } else {
                    Some((x.clone(), y - 1))
                }
            }
            Move::Left => {
                if *x == 0 {
                    None
                } else {
                    Some((x - 1, y.clone()))
                }
            }
            Move::Right => Some((x + 1, y.clone())),
            Move::Down => Some((x.clone(), y + 1)),
        }
    }
    fn inverse_transform(&self, coords: &(usize, usize)) -> Option<(usize, usize)> {
        match self {
            Move::Up => Move::Down.transform(coords),
            Move::Left => Move::Right.transform(coords),
            Move::Right => Move::Left.transform(coords),
            Move::Down => Move::Up.transform(coords),
        }
    }
    fn horizontal(&self) -> bool {
        *self == Move::Left || *self == Move::Right
    }
}

fn read_moves(s: &str) -> Vec<Move> {
    s.chars()
        .filter(|ch| *ch != '\n')
        .map(|ch| From::from(ch))
        .collect_vec()
}

fn read_input(input: &str, question: Question) -> Result<(Map, Vec<Move>), String> {
    let (map, moves) = input
        .split("\n\n")
        .collect_tuple()
        .ok_or("can't split into map n moves")?;
    let map = Map::read_map(map, question)?;
    let moves = read_moves(moves);
    Ok((map, moves))
}

impl Map {
    fn get(&self, coords: &(usize, usize)) -> Pixel {
        let (x, y) = coords;
        self.map[*y][*x]
    }
    fn set(&mut self, (x, y): (usize, usize), px: Pixel) {
        self.map[y][x] = px;
    }
    fn next_step(&mut self, mv: Move) {
        let mut first_empty = mv.transform(&self.robot);
        while first_empty.is_some() && self.get(&first_empty.unwrap()).moveable() {
            first_empty = mv.transform(&first_empty.unwrap());
        }
        if first_empty.is_none() {
            return;
        }
        let mut first_empty = first_empty.unwrap();
        if self.get(&first_empty) == Pixel::Empty {
            // moveables
            while self.get(&first_empty) != Pixel::Robot {
                let prev = mv.inverse_transform(&first_empty).unwrap();
                self.set(first_empty, self.get(&prev));
                first_empty = prev;
            }
            // clean up
            self.set(first_empty, Pixel::Empty);
            self.robot = mv.transform(&first_empty).unwrap();
        }
    }
    fn box_coords(&self) -> u128 {
        self.map
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, p)| match *p {
                    Pixel::Box => Some((i * 100 + j) as u128),
                    Pixel::BoxLeft => Some((i * 100 + j) as u128),
                    _ => None,
                })
            })
            .flatten()
            .sum()
    }
    fn q_2_next_step(&mut self, mv: Move) {
        if let Some(moveables) = self.q_2_moveable(&self.robot, mv) {
            let mut new_map = self.map.clone();
            for moveable in moveables {
                new_map[moveable.1][moveable.0] =
                    self.get(&mv.inverse_transform(&moveable).unwrap())
            }
            self.map = new_map;
            self.set(self.robot, Pixel::Empty);
            // necessary cleanup babahahahahahahah
            for j in 0..self.map.len() {
                for i in 0..self.map[j].len() - 1 {
                    if self.map[j][i] == Pixel::BoxLeft && self.map[j][i + 1] != Pixel::BoxRight {
                        self.map[j][i] = Pixel::Empty;
                    }
                    if self.map[j][i] != Pixel::BoxLeft && self.map[j][i + 1] == Pixel::BoxRight {
                        self.map[j][i + 1] = Pixel::Empty;
                    }
                }
            }
            if !mv.horizontal() {
                if self.get(&(self.robot.0 + 1, self.robot.1)) == Pixel::BoxRight {
                    self.map[self.robot.1][self.robot.0 + 1] = Pixel::Empty;
                }

                if self.get(&(self.robot.0 - 1, self.robot.1)) == Pixel::BoxLeft {
                    self.map[self.robot.1][self.robot.0 - 1] = Pixel::Empty;
                }
            }
            self.robot = mv.transform(&self.robot).unwrap();
        }
    }
    fn q_2_moveable(
        &self,
        coords: &(usize, usize),
        direction: Move,
    ) -> Option<HashSet<(usize, usize)>> {
        match direction.transform(coords) {
            None => None,
            Some(px) => match self.get(&px) {
                Pixel::Wall => None,
                Pixel::Empty => Some(HashSet::from([px.clone()])),
                Pixel::Box => {
                    if let Some(moved_ones) = self.q_2_moveable(&px, direction) {
                        let result = moved_ones
                            .union(&HashSet::from([px.clone()]))
                            .map(|x| *x)
                            .collect();
                        Some(result)
                    } else {
                        None
                    }
                }
                Pixel::Robot => panic!("what's going on {:?} {:?}", px, coords),
                Pixel::BoxLeft if direction.horizontal() => {
                    self.q_2_moveable(&px, direction).map(|set| {
                        set.union(&HashSet::from([px.clone()]))
                            .map(|x| *x)
                            .collect()
                    })
                }
                Pixel::BoxRight if direction.horizontal() => {
                    self.q_2_moveable(&px, direction).map(|set| {
                        set.union(&HashSet::from([px.clone()]))
                            .map(|x| *x)
                            .collect()
                    })
                }
                Pixel::BoxLeft => {
                    let (x, y) = px.clone();
                    let mine = self.q_2_moveable(&px, direction);
                    let neighbor = self.q_2_moveable(&(x + 1, y), direction.clone());
                    if let (Some(mine), Some(neighbor)) = (mine, neighbor) {
                        Some(
                            mine.union(&neighbor)
                                .map(|x| *x)
                                .collect::<HashSet<(usize, usize)>>()
                                .union(&HashSet::from([px.clone()]))
                                .map(|x| *x)
                                .collect(),
                        )
                    } else {
                        None
                    }
                }
                Pixel::BoxRight => {
                    let (x, y) = px.clone();
                    let mine = self.q_2_moveable(&px, direction);
                    let neighbor = self.q_2_moveable(&(x - 1, y), direction.clone());
                    if let (Some(mine), Some(neighbor)) = (mine, neighbor) {
                        Some(
                            mine.union(&neighbor)
                                .map(|x| *x)
                                .collect::<HashSet<(usize, usize)>>()
                                .union(&HashSet::from([px.clone()]))
                                .map(|x| *x)
                                .collect(),
                        )
                    } else {
                        None
                    }
                }
            },
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for pix in row {
                write!(f, "{}", pix);
            }
            writeln!(f);
        }
        Ok(())
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let (mut map, moves) = read_input(input, question)?;
    for mv in moves {
        // println!("move {:?}", mv);
        // println!("rob {:?}", map.robot);
        // println!("{}", map);
        map.q_2_next_step(mv);
    }
    Ok(map.box_coords())
}
