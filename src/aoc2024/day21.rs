// doesn't work bc dorsnt' find the optimal -- it's not even looking

use crate::common::day::{Day, Question};
use itertools::{repeat_n, Itertools};
use std::collections::HashMap;

pub struct Day21;

impl Day for Day21 {
    fn question(&self, input: &str, question: Question) {
        let tasks = input
            .lines()
            .map(|task| task.chars().map(|ch| Button::from(ch)).collect_vec())
            .map(|mv| prepend_a(mv))
            .collect_vec();
        let main_board = Board {
            map: HashMap::from([
                (Button::Digit(7), (0, 0)),
                (Button::Digit(8), (1, 0)),
                (Button::Digit(9), (2, 0)),
                (Button::Digit(4), (0, 1)),
                (Button::Digit(5), (1, 1)),
                (Button::Digit(6), (2, 1)),
                (Button::Digit(1), (0, 2)),
                (Button::Digit(2), (1, 2)),
                (Button::Digit(3), (2, 2)),
                (Button::Digit(0), (1, 3)),
                (Button::A, (2, 3)),
            ]),
        };
        let control_board = Board {
            map: HashMap::from([
                (Button::Up, (1, 0)),
                (Button::A, (2, 0)),
                (Button::Left, (0, 1)),
                (Button::Down, (1, 1)),
                (Button::Right, (2, 1)),
            ]),
        };
        let mut result = 0;
        for task in tasks {
            let raw = raw_num(&task);
            let moves = main_board.run_task(task);
            println!("{}", print_moves(&moves));
            let moves2 = control_board.run_task(prepend_a(moves));
            println!("{}", print_moves(&moves2));
            let moves3 = control_board.run_task(prepend_a(moves2));
            println!("{}", print_moves(&moves3));
            println!("{}", moves3.len());
            println!("{}", raw);
            result += moves3.len() as u128 * raw;

            println!("Reversing...");
            let removes2 = control_board.play_moves(moves3);
            if removes2.is_none() {
                println!("Megafail");
                continue;
            }
            let removes2 = removes2.unwrap();
            println!("{}", print_moves(&removes2));
            let removes1 = control_board.play_moves(removes2);
            if removes1.is_none() {
                println!("Megafail");
                continue;
            }
            let removes1 = removes1.unwrap();
            println!("{}", print_moves(&removes1));
            let reraw = main_board.play_moves(removes1);
            if reraw.is_none() {
                println!("Megafail");
                continue;
            }
            println!("{}", print_moves(&reraw.unwrap()));

            println!();
        }

        println!("{}", result);
    }

    fn test_data(&self) -> String {
        "029A
980A
179A
456A
379A"
            .to_string()
    }
}

type Coord = (isize, isize);

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    Digit(usize),
}

impl From<char> for Button {
    fn from(ch: char) -> Self {
        match ch {
            n if n >= '0' && n <= '9' => Button::Digit(n.to_digit(10).unwrap() as usize),
            'A' => Button::A,
            _ => panic!("uuuf"),
        }
    }
}

impl ToString for Button {
    fn to_string(&self) -> String {
        match self {
            Button::Up => "^".to_string(),
            Button::Down => "v".to_string(),
            Button::Left => "<".to_string(),
            Button::Right => ">".to_string(),
            Button::A => "A".to_string(),
            Button::Digit(n) => n.to_string(),
        }
    }
}

struct Board {
    map: HashMap<Button, Coord>,
}

impl Board {
    fn moves(&self, from: &Button, to: &Button) -> Vec<Button> {
        let from_coord = self.map.get(from).unwrap();
        let to_coord = self.map.get(to).unwrap();
        let dx = to_coord.0 - from_coord.0;
        let dy = to_coord.1 - from_coord.1;
        let xes = repeat_n(
            if dx < 0 { Button::Left } else { Button::Right },
            dx.abs() as usize,
        )
        .collect_vec();
        let ys = repeat_n(
            if dy < 0 { Button::Up } else { Button::Down },
            dy.abs() as usize,
        )
        .collect_vec();
        // xes ys is perfect but hovers over panic
        // ys xes is not optimal as < should be first
        // ugh hv to avoid that gap
        match from {
            Button::Left | Button::Digit(1) | Button::Digit(4) | Button::Digit(7) => {
                vec![xes, ys].concat()
            }
            _ => vec![ys, xes].concat(),
        }
    }
    fn multi_moves(&self, from: &Button, to: &Button) -> Vec<Vec<Button>> {
        let from_coord = self.map.get(from).unwrap();
        let to_coord = self.map.get(to).unwrap();
        let dx = to_coord.0 - from_coord.0;
        let dy = to_coord.1 - from_coord.1;
        let xes = repeat_n(
            if dx < 0 { Button::Left } else { Button::Right },
            dx.abs() as usize,
        )
        .collect_vec();
        let ys = repeat_n(
            if dy < 0 { Button::Up } else { Button::Down },
            dy.abs() as usize,
        )
        .collect_vec();

        let all_moves = vec![xes, ys].concat();
        todo!()
        // xes.
        // // xes ys is perfect but hovers over panic
        // // ys xes is not optimal as < should be first
        // // ugh hv to avoid that gap
        // match from {
        //     Button::Left | Button::Digit(1) | Button::Digit(4) | Button::Digit(7) => {
        //         vec![xes, ys].concat()
        //     }
        //     _ => vec![ys, xes].concat(),
        // }
    }
    fn run_task(&self, task: Vec<Button>) -> Vec<Button> {
        let moves = task
            .iter()
            .tuple_windows()
            .map(|(from, to)| {
                let moves = self.moves(from, to);
                let moves = vec![moves, vec![Button::A]].concat();
                moves
            })
            .flatten()
            .collect_vec();
        moves
    }

    fn play_moves(&self, moves: Vec<Button>) -> Option<Vec<Button>> {
        let mut output = vec![];
        let mut pointing = self.map.get(&Button::A).unwrap().clone();
        for mv in moves {
            match mv {
                Button::Up => pointing.1 -= 1,
                Button::Down => pointing.1 += 1,
                Button::Left => pointing.0 -= 1,
                Button::Right => pointing.0 += 1,
                Button::A => {
                    let but = self.map.iter().find(|(butt, coord)| **coord == pointing);
                    if but.is_none() {
                        return None;
                    }
                    output.push(but.unwrap().0.clone());
                }
                Button::Digit(_) => return None,
            }
        }
        Some(output)
    }
}

fn print_moves(moves: &Vec<Button>) -> String {
    moves.iter().map(|mv| mv.to_string()).join("")
}
fn prepend_a(moves: Vec<Button>) -> Vec<Button> {
    vec![vec![Button::A], moves].concat()
}

fn raw_num(moves: &Vec<Button>) -> u128 {
    moves
        .iter()
        .enumerate()
        .map(|(i, mv)| match mv {
            Button::Digit(n) => *n * 10usize.pow(3 - i as u32),
            _ => 0,
        })
        .sum::<usize>() as u128
}
