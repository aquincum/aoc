// ABORTED because I didn't read and this works for rows & columns but diagonals... I didn't read

use crate::common::day::{Day, Question};
use crate::common::map::Map;
use std::ops::Range;

pub struct Day4;

impl Day for Day4 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string()
    }
}

enum State {
    X,
    M,
    A,
    S,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Row,
    Column,
}

#[derive(Copy, Clone)]
struct Params {
    first_dimension: usize,
    second_dimension: usize,
    second_reverse: bool,
}

fn do_task<F>(params: Params, get_value: F) -> u128
where
    F: Fn(usize, usize) -> char,
{
    println!("---");
    (0..params.first_dimension)
        .map(|i| {
            let (finds, _) = (0..params.second_dimension).clone().fold(
                (0u128, State::X),
                |(finds, state), j| {
                    let j = if params.second_reverse {
                        params.second_dimension - j - 1
                    } else {
                        j
                    };
                    let point = get_value(i, j);
                    println!("{} {} {}", i, j, point);
                    match (point, state) {
                        ('X', _) => (finds, State::M),
                        ('M', State::M) => (finds, State::A),
                        ('A', State::A) => (finds, State::S),
                        ('S', State::S) => (finds + 1, State::X),
                        _ => (finds, State::X),
                    }
                },
            );
            finds
        })
        .sum()
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let map: Map<char> = input.parse()?;
    let n = do_task(
        Params {
            first_dimension: map.height(),
            second_dimension: map.width(),
            second_reverse: false,
        },
        |i, j| map.get_no_floor(j, i),
    ) + do_task(
        Params {
            first_dimension: map.height(),
            second_dimension: map.width(),
            second_reverse: true,
        },
        |i, j| map.get_no_floor(j, i),
    ) + do_task(
        Params {
            first_dimension: map.width(),
            second_dimension: map.height(),
            second_reverse: false,
        },
        |i, j| map.get_no_floor(i, j),
    ) + do_task(
        Params {
            first_dimension: map.width(),
            second_dimension: map.height(),
            second_reverse: true,
        },
        |i, j| map.get_no_floor(i, j),
    );
    Ok(n)
}
