use crate::aoc2022::day14::Object;
use crate::common::lines::Lineser;
use itertools::Itertools;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub trait Bottom {
    fn bottom() -> Self;
}

#[derive(Clone)]
pub struct Map<T> {
    points: Vec<T>,
    min_x: usize,
    max_x: usize,
    max_y: usize,
    min_y: usize,
    floor: bool,
    rotated: bool,
}

impl<T: Display + Default + Clone> Map<T> {
    pub fn new(min_x: usize, max_x: usize, max_y: usize, floor: bool) -> Self {
        let mut map = Map {
            points: Vec::new(),
            min_x: min_x - 5,
            max_x: max_x + 5,
            min_y: 0,
            max_y: max_y + 1,
            floor,
            rotated: false,
        };
        map.points = vec![Default::default(); map.vec_len()];
        map
    }
    pub fn width(&self) -> usize {
        self.max_x - self.min_x + 1
    }
    pub fn height(&self) -> usize {
        self.max_y - self.min_y + 1
    }
    pub fn vec_len(&self) -> usize {
        self.width() * self.height()
    }
    fn index(&self, i: usize, j: usize) -> usize {
        if i < self.min_x {
            print!("I {}", i);
            println!("{}", self);
        }
        let idx_x = if !self.rotated {
            i - self.min_x
        } else {
            j - self.min_x
        };
        let idx_y = if !self.rotated {
            self.width() * j - self.min_y * self.width()
        } else {
            self.width() * (self.width() - i - 1) - self.min_y * self.width()
        };
        idx_x + idx_y
    }
    pub fn set(&mut self, i: usize, j: usize, obj: T) {
        let idx = self.index(i, j);
        self.points[idx] = obj;
    }
    pub fn get_ref(&self, i: usize, j: usize) -> &T {
        &self.points[self.index(i, j)]
    }
    pub fn rotate(&self) -> Map<T> {
        Map {
            points: self.points.clone(),
            min_x: self.min_y,
            max_x: self.max_y,
            min_y: self.min_x,
            max_y: self.max_x,
            floor: self.floor,
            rotated: !self.rotated,
        }
    }
}

impl FromStr for Map<char> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<char> = s.chars().filter(|&c| c != '\n').collect_vec();
        let lines = s.lines().collect_vec();
        let max_y = lines.len() - 1;
        let max_x = lines[0].len() - 1;

        Ok(Map {
            points,
            floor: false,
            min_x: 0,
            min_y: 0,
            max_x,
            max_y,
            rotated: false,
        })
    }
}

// impl<T: From<char>> FromStr for Map<T> {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let points: Vec<char> = s.chars().filter(|&c| c != '\n').collect_vec();
//         let points = points.into_iter().map(|c| From::from(c)).collect_vec();
//         let lines = s.lines().collect_vec();
//         let max_y = lines.len() - 1;
//         let max_x = lines[0].len() - 1;
//
//         Ok(Map {
//             points,
//             floor: false,
//             min_x: 0,
//             min_y: 0,
//             max_x,
//             max_y,
//             rotated: false,
//         })
//     }
// }

impl<T: Display + Default + Clone + Copy> Map<T> {
    pub fn get_no_floor(&self, i: usize, j: usize) -> T {
        self.points[self.index(i, j)]
    }
}

impl<T: Display + Default + Clone + Bottom + Copy> Map<T> {
    pub fn get(&self, i: usize, j: usize) -> T {
        if self.floor && j == self.max_y {
            return Bottom::bottom();
        }
        let idx = self.index(i, j);
        self.points[idx]
    }
}

impl Map<Object> {
    pub fn drop(&mut self, drop_x: usize, drop_y: usize) -> bool {
        let mut x = drop_x;
        let mut y = drop_y;
        loop {
            if y >= self.max_y {
                return true;
            }
            if self.get(x, y + 1) == Object::Air {
                y += 1;
            } else if self.get(x - 1, y + 1) == Object::Air {
                x -= 1;
                y += 1;
            } else if self.get(x + 1, y + 1) == Object::Air {
                x += 1;
                y += 1;
            } else {
                self.set(x, y, Object::Sand);
                break;
            }
        }
        false
    }
}

impl<T: Display + Default + Clone> Display for Map<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.floor && !self.rotated {
            for (i, elem) in self.points.iter().enumerate() {
                if i >= self.max_y * self.width() {
                    write!(f, "F");
                } else {
                    write!(f, "{}", elem);
                    if i % self.width() == self.width() - 1 {
                        writeln!(f);
                    }
                }
            }
        } else if self.floor && self.rotated {
            writeln!(f, "rotated floor is just too much");
        } else {
            for x in 0..self.width() {
                for y in 0..self.height() {
                    write!(f, "{}", self.points[self.index(x, y)]);
                }
                writeln!(f);
            }
        }
        write!(f, "")
    }
}
