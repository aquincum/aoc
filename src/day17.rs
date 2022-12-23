use std::ops::Add;
use std::fmt::{Display, Formatter};
use std::io::{stdin, Read};

const WIDTH: usize = 7;

type Bitmap = i64

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug)]
struct Coordinate {
    x: i8,
    y: i128,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Coordinate {
    fn index(&self) -> usize {
        ((self.y-1) as usize)*WIDTH+((self.x-1) as usize)
    }
    fn bitmap(&self, bottom_y: i128) -> Bitmap {
        (1 << (WIDTH-self.x)) << ((self.y-bottom_y) * WIDTH)
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
                self.bottom_left+Coordinate{x:1, y:0},
                self.bottom_left+Coordinate{x:2, y:0},
                self.bottom_left+Coordinate{x:3, y:0},
            ],
            TetrisShape::Plus => vec![
                self.bottom_left+Coordinate{x:0, y:1},
                self.bottom_left+Coordinate{x:1, y:1},
                self.bottom_left+Coordinate{x:2, y:1},
                self.bottom_left+Coordinate{x:1, y:0},
                self.bottom_left+Coordinate{x:1, y:2},

            ],
            TetrisShape::El => vec![
                self.bottom_left,
                self.bottom_left+Coordinate{x:1, y:0},
                self.bottom_left+Coordinate{x:2, y:0},
                self.bottom_left+Coordinate{x:2, y:1},
                self.bottom_left+Coordinate{x:2, y:2},
            ],
            TetrisShape::VLine => vec![
                self.bottom_left,
                self.bottom_left+Coordinate{x:0, y:1},
                self.bottom_left+Coordinate{x:0, y:2},
                self.bottom_left+Coordinate{x:0, y:3},
            ],
            TetrisShape::Box => vec![
                self.bottom_left,
                self.bottom_left+Coordinate{x:1, y:0},
                self.bottom_left+Coordinate{x:0, y:1},
                self.bottom_left+Coordinate{x:1, y:1},
            ]
        }
    }
    fn top(&self) -> Coordinate {
        match self.shape {
            TetrisShape::HLine => self.bottom_left+Coordinate{x:3, y:0},
            TetrisShape::Plus =>
                self.bottom_left+Coordinate{x:1, y:2},
            TetrisShape::El =>
                self.bottom_left+Coordinate{x:2, y:2},
            TetrisShape::VLine =>
                self.bottom_left+Coordinate{x:0, y:3},
            TetrisShape::Box =>
                self.bottom_left+Coordinate{x:1, y:1},
        }
    }
    fn shift(&self, by: Coordinate, field: &Field) -> Option<Self> {
        let new_tetris = Tetris {
            bottom_left: self.bottom_left + by,
            shape: self.shape,
        };
        if new_tetris.pixels().iter().any(|px| field.get(px)) {
            None
        } else {
            Some(new_tetris)
        }
    }
    fn bitmap(&self, bottom_y: i128) -> Bitmap {
        match self.shape {
            TetrisShape::HLine => 0b1111 << (4 - self.bottom_left.x) << WIDTH*(self.bottom_left.y - bottom_y),
            TetrisShape::Plus =>  0b1000001110000010 << (5 - self.bottom_left.x) << WIDTH*(self.bottom_left.y - bottom_y),
            TetrisShape::El => 0b100000010000111 << (5 - self.bottom_left.x) << WIDTH*(self.bottom_left.y - bottom_y),
            TetrisShape::VLine => 0b1000000100000010000001 << (7 - self.bottom_left.x) << WIDTH*(self.bottom_left.y - bottom_y),
            TetrisShape::Box => 0b00000110000011 << (6- - self.bottom_left.x) << WIDTH*(self.bottom_left.y - bottom_y)
        }
    }
}

// x: 012345678 -- 0 and 8 are walls
// y: grows upwards
struct Field {
    field: Bitmap,
    bottom_y: i128
};

impl Field {
    fn new() -> Self {
        Field{
            field: 0,
            bottom_y: 0,
        }
    }
    fn get(&self, c: &Coordinate) -> bool {
        if c.x == 0 || c.x as usize == WIDTH + 1 || c.y == 0 {
            return true;
        }
        self.field
        self.0.get(c.index()).unwrap_or_else(|| &false).clone()
    }
    fn set_in_stone(&mut self, t: Tetris) {
        let top = t.top().index();
        if top >= self.0.len() {
            self.0.resize(top + WIDTH *3, false);
        }
        for px in t.pixels() {
            self.0[px.index()] = true;
        }
    }
    fn top_y(&self) -> i128 {
        let from_back = self.0.iter().rev().position(|t| *t);
        match from_back {
            None => 0,
            Some(n) => ((self.0.len() - n -1) / WIDTH + 1) as i128
        }
    }
}

fn print_state(field: &Field, tetris: Option<Tetris>) {
    let top_draw = tetris.map(|t| t.top().y+1).unwrap_or(field.top_y()+1);
    for y in (0..(top_draw+1)).rev() {
        for x in 0..9 {
            if tetris.map(|t| t.pixels().iter().any(|px| px == &Coordinate{x,y})).unwrap_or(false) {
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
            } else if field.get(&Coordinate{x,y}) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
}


pub fn question(input: &str){
    let mut field = Field::new();
    let mut active_tetris = None;
    let mut shape_rotation: i128 = 0;
    for ch in input.chars().cycle() {
        if active_tetris.is_none() {
            active_tetris = Some(Tetris{
                shape: match shape_rotation % 5 {
                    0 => TetrisShape::HLine,
                    1 => TetrisShape::Plus,
                    2 => TetrisShape::El,
                    3 => TetrisShape::VLine,
                    4 => TetrisShape::Box,
                    _ => panic!("Mathematical hilarity") // ??
                },
                bottom_left: Coordinate {
                    x: 3,
                    y: field.top_y() + 4,
                }
            });
            if shape_rotation < 20 {
                println!("\nRock {} begins falling:", shape_rotation);
                print_state(&field, active_tetris);
            }
        }
        let h_move = match ch {
            '<' => Coordinate{x: -1, y:0},
            '>' => Coordinate{x: 1, y: 0},
            _ => panic!("Non-<> char: {}", ch)
        };
        if let Some(after_h_move) = active_tetris.unwrap().shift(h_move, &field) {
            active_tetris = Some(after_h_move);
            if shape_rotation < 20 {
                println!("\nJet of gas pushes rock to side:");
                print_state(&field, active_tetris);
                pause()
            }
        } else if shape_rotation < 20{
            println!("\nJet of gas pushes rock to side, but nothing happens");
            print_state(&field, active_tetris);
            pause()
        }
        let drop = Coordinate{x: 0, y: -1};
        let after_drop = active_tetris.unwrap().shift(drop, &field);
        if after_drop.is_some() {
            active_tetris = after_drop;
            if shape_rotation < 20 {
                println!("\nRock falls:");
                print_state(&field, active_tetris);
                pause()
            }
        } else {
            field.set_in_stone(active_tetris.unwrap());
            active_tetris = None;
            shape_rotation += 1;
            if shape_rotation == 2022 {
                break;
            }
            // println!("Top now at {}: {}", shape_rotation, field.top_y());
            if shape_rotation <= 20 {
                println!("\n***TOP NOW AT {}", field.top_y());
                println!("Rock falls 1 unit, causing it to come to rest:");
                print_state(&field, active_tetris);
                pause()
            }

        }
    }
    // println!("\n\n\nEND =================================================");
    // print_state(&field, None);
    println!("Top at end: {}" , field.top_y());
}
//3097 too low
fn pause() {
    // stdin().read(&mut [0]).unwrap();
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

mod q2 {
    fn q2(input: &str) {

    }
}