use crate::day14::Object;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::net::Shutdown::Both;

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
        let idx_x = i - self.min_x;
        let idx_y = self.width() * j - self.min_y * self.width();
        idx_x + idx_y
    }
    pub fn set(&mut self, i: usize, j: usize, obj: T) {
        let idx = self.index(i, j);
        self.points[idx] = obj;
    }
    pub fn get_ref(&self, i: usize, j: usize) -> &T {
        &self.points[self.index(i, j)]
    }
}

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
        for (i, elem) in self.points.iter().enumerate() {
            if self.floor && i >= self.max_y * self.width() {
                write!(f, "F");
            } else {
                write!(f, "{}", elem);
                if i % self.width() == self.width() - 1 {
                    writeln!(f);
                }
            }
        }
        write!(f, "aah")
    }
}
