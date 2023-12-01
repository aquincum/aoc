use crate::common::day::{Day, Question};
use std::fmt::{Display, Formatter};
use std::io::{stdin, Read};
use std::ops::Add;

const WIDTH: usize = 7;
const ROW_MEMORY: usize = 13;

type Bitmap = u128;

pub struct Day17;
impl Day for Day17 {
    fn question(&self, input: &str, question: Question) {
        self::question(
            input,
            match question {
                Question::First => 2022,
                Question::Second => 1000000000000,
            },
        );
    }

    fn test_data(&self) -> String {
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug)]
struct Coordinate {
    x: i8,
    y: i128,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Coordinate {
    fn index(&self) -> usize {
        ((self.y - 1) as usize) * WIDTH + ((self.x - 1) as usize)
    }
    fn bitmap(&self, bottom_y: i128) -> Bitmap {
        // println!("y={} botty={}", self.y, bottom_y);
        (1 << (WIDTH as i8 - self.x)) << ((self.y - bottom_y) * WIDTH as i128)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum TetrisShape {
    HLine,
    Plus,
    El,
    VLine,
    Box,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Tetris {
    shape: TetrisShape,
    bottom_left: Coordinate,
}

impl Tetris {
    fn pixels(&self) -> Vec<Coordinate> {
        match self.shape {
            TetrisShape::HLine => vec![
                self.bottom_left,
                self.bottom_left + Coordinate { x: 1, y: 0 },
                self.bottom_left + Coordinate { x: 2, y: 0 },
                self.bottom_left + Coordinate { x: 3, y: 0 },
            ],
            TetrisShape::Plus => vec![
                self.bottom_left + Coordinate { x: 0, y: 1 },
                self.bottom_left + Coordinate { x: 1, y: 1 },
                self.bottom_left + Coordinate { x: 2, y: 1 },
                self.bottom_left + Coordinate { x: 1, y: 0 },
                self.bottom_left + Coordinate { x: 1, y: 2 },
            ],
            TetrisShape::El => vec![
                self.bottom_left,
                self.bottom_left + Coordinate { x: 1, y: 0 },
                self.bottom_left + Coordinate { x: 2, y: 0 },
                self.bottom_left + Coordinate { x: 2, y: 1 },
                self.bottom_left + Coordinate { x: 2, y: 2 },
            ],
            TetrisShape::VLine => vec![
                self.bottom_left,
                self.bottom_left + Coordinate { x: 0, y: 1 },
                self.bottom_left + Coordinate { x: 0, y: 2 },
                self.bottom_left + Coordinate { x: 0, y: 3 },
            ],
            TetrisShape::Box => vec![
                self.bottom_left,
                self.bottom_left + Coordinate { x: 1, y: 0 },
                self.bottom_left + Coordinate { x: 0, y: 1 },
                self.bottom_left + Coordinate { x: 1, y: 1 },
            ],
        }
    }
    fn top(&self) -> Coordinate {
        match self.shape {
            TetrisShape::HLine => self.bottom_left + Coordinate { x: 3, y: 0 },
            TetrisShape::Plus => self.bottom_left + Coordinate { x: 1, y: 2 },
            TetrisShape::El => self.bottom_left + Coordinate { x: 2, y: 2 },
            TetrisShape::VLine => self.bottom_left + Coordinate { x: 0, y: 3 },
            TetrisShape::Box => self.bottom_left + Coordinate { x: 1, y: 1 },
        }
    }
    fn shift(&self, by: Coordinate, field: &Field) -> Option<Self> {
        let new_tetris = Tetris {
            bottom_left: self.bottom_left + by,
            shape: self.shape,
        };
        if field.collides(&new_tetris) {
            None
        } else {
            Some(new_tetris)
        }
    }
    fn bitmap(&self, bottom_y: i128) -> Bitmap {
        let (bm, _) = match self.shape {
            TetrisShape::HLine => (0b1111u128 << (4 - self.bottom_left.x))
                .overflowing_shl(((WIDTH as i128) * (self.bottom_left.y - bottom_y)) as u32),
            TetrisShape::Plus => (0b1000001110000010u128 << (5 - self.bottom_left.x))
                .overflowing_shl(((WIDTH as i128) * (self.bottom_left.y - bottom_y)) as u32),
            TetrisShape::El => (0b100000010000111u128 << (5 - self.bottom_left.x))
                .overflowing_shl(((WIDTH as i128) * (self.bottom_left.y - bottom_y)) as u32),
            TetrisShape::VLine => (0b1000000100000010000001u128 << (7 - self.bottom_left.x))
                .overflowing_shl(((WIDTH as i128) * (self.bottom_left.y - bottom_y)) as u32),
            TetrisShape::Box => (0b00000110000011u128 << (6 - self.bottom_left.x))
                .overflowing_shl(((WIDTH as i128) * (self.bottom_left.y - bottom_y)) as u32),
        };
        bm
    }
    fn width(&self) -> i8 {
        match self.shape {
            TetrisShape::HLine => 4,
            TetrisShape::Plus => 3,
            TetrisShape::El => 3,
            TetrisShape::VLine => 1,
            TetrisShape::Box => 2,
        }
    }
}

// x: 012345678 -- 0 and 8 are walls
// y: grows upwards
struct Field {
    field: Bitmap,
    bottom_y: i128,
}

impl Field {
    fn new() -> Self {
        Field {
            field: 0,
            bottom_y: 1,
        }
    }
    fn get(&self, c: &Coordinate) -> bool {
        if c.x == 0 || c.x as usize == WIDTH + 1 || c.y == 0 {
            return true;
        }
        c.bitmap(self.bottom_y) & self.field != 0
    }
    fn collides(&self, t: &Tetris) -> bool {
        if t.bottom_left.x == 0
            || t.bottom_left.x + t.width() > (WIDTH + 1) as i8
            || t.bottom_left.y == 0
        {
            return true;
        }
        t.bitmap(self.bottom_y) & self.field != 0
    }
    fn set_in_stone(&mut self, t: Tetris) {
        let shape = t.bitmap(self.bottom_y);
        self.field |= shape;
        if self.field > ((1 as i128) << WIDTH * ROW_MEMORY) as Bitmap {
            let (extrarows, _) = self.field.overflowing_shr((WIDTH * ROW_MEMORY) as u32);
            let fl: f32 = extrarows as f32;
            let extrarows = fl.log(WIDTH as f32).floor() as u32;
            println!(
                "Shifting: by={} er={} filed={:#10b}",
                self.bottom_y, extrarows, self.field
            );
            self.field >>= extrarows * (WIDTH as u32);
            self.bottom_y += extrarows as i128;
        }
    }
    fn top_y(&self) -> i128 {
        let mut f = self.field;
        let mut y = self.bottom_y;
        while f != 0 {
            f >>= WIDTH;
            y += 1;
        }
        y - 1
    }
}

fn print_state(field: &Field, tetris: Option<Tetris>) {
    let top_draw = tetris.map(|t| t.top().y + 1).unwrap_or(field.top_y() + 1);
    for y in (field.bottom_y..(top_draw + 1)).rev() {
        for x in 0..9 {
            if tetris
                .map(|t| t.pixels().iter().any(|px| px == &Coordinate { x, y }))
                .unwrap_or(false)
            {
                print!("@");
            } else if x == 0 {
                print!("|");
            } else if x == 8 {
                if y % 5 == 0 {
                    println!("| {}", y)
                } else {
                    println!("|");
                }
            } else if y == 0 {
                print!("-");
            } else if field.get(&Coordinate { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
}

pub fn question(input: &str, rounds: i128) {
    let mut field = Field::new();
    let mut active_tetris = None;
    let mut shape_rotation: i128 = 0;
    const DEBUG_LEN: i128 = 5;
    for ch in input.chars().cycle() {
        if active_tetris.is_none() {
            active_tetris = Some(Tetris {
                shape: match shape_rotation % 5 {
                    0 => TetrisShape::HLine,
                    1 => TetrisShape::Plus,
                    2 => TetrisShape::El,
                    3 => TetrisShape::VLine,
                    4 => TetrisShape::Box,
                    _ => panic!("Mathematical hilarity"), // ??
                },
                bottom_left: Coordinate {
                    x: 3,
                    y: field.top_y() + 4,
                },
            });
            if shape_rotation < DEBUG_LEN {
                println!("\nRock {} begins falling:", shape_rotation);
                print_state(&field, active_tetris);
            }
        }
        let h_move = match ch {
            '<' => Coordinate { x: -1, y: 0 },
            '>' => Coordinate { x: 1, y: 0 },
            _ => panic!("Non-<> char: {}", ch),
        };
        if let Some(after_h_move) = active_tetris.unwrap().shift(h_move, &field) {
            active_tetris = Some(after_h_move);
            if shape_rotation < DEBUG_LEN {
                println!("\nJet of gas pushes rock to side:");
                print_state(&field, active_tetris);
                pause()
            }
        } else if shape_rotation < DEBUG_LEN {
            println!("\nJet of gas pushes rock to side, but nothing happens");
            print_state(&field, active_tetris);
            pause()
        }
        let drop = Coordinate { x: 0, y: -1 };
        let after_drop = active_tetris.unwrap().shift(drop, &field);
        if after_drop.is_some() {
            active_tetris = after_drop;
            if shape_rotation < DEBUG_LEN {
                println!("\nRock falls:");
                print_state(&field, active_tetris);
                pause()
            }
        } else {
            field.set_in_stone(active_tetris.unwrap());
            active_tetris = None;
            shape_rotation += 1;
            if shape_rotation == rounds {
                break;
            }
            println!("Top now at {}: {}", shape_rotation, field.top_y());
            if shape_rotation <= DEBUG_LEN {
                println!("\n***TOP NOW AT {}", field.top_y());
                println!("Rock falls 1 unit, causing it to come to rest:");
                print_state(&field, active_tetris);
                pause()
            }
        }
    }
    // println!("\n\n\nEND =================================================");
    // print_state(&field, None);
    println!("Top at end: {}", field.top_y());
}
//3097 too low
fn pause() {
    // stdin().read(&mut [0]).unwrap();
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
