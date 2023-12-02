use std::fmt::format;
use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;
use itertools::Itertools;
use crate::aoc2023::day1::Day1;
use crate::common::day::{Day, Question};

pub struct Day2;

impl Day for Day2{
    fn question(&self, input: &str, question: Question) {
        let result = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{}", result);
    }

    fn test_data(&self) -> String {
        return "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
    }
}

#[derive(Debug)]

struct Draw {
    blues: u32,
    reds: u32,
    greens: u32
}

impl Add for Draw {
    type Output = Draw;

    fn add(self, rhs: Self) -> Self::Output {
        Draw {
            blues: self.blues + rhs.blues,
            reds: self.reds + rhs.reds,
            greens: self.greens + rhs.greens,
        }
    }
}

impl FromStr for Draw {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sets = s.split(", ");
        sets.fold(Ok(Draw{
            blues: 0,
            reds: 0,
            greens: 0,
        }), |acc, item| {
            if acc.is_err() {
                return acc;
            }
            if let Some((n_str, color)) = item.split(" ").collect_tuple() {
                let n = n_str.parse().map_err(|e: ParseIntError| e.to_string())?;
                match color {
                    "red" => Ok(acc.unwrap() + Draw{reds: n, blues: 0, greens: 0}),
                    "blue" => Ok(acc.unwrap() + Draw{reds: 0, blues: n, greens: 0}),
                    "green" => Ok(acc.unwrap() + Draw{reds: 0, blues: 0, greens: n}),
                    _ => Err(format!("{} is not a color", color))
                }
            } else {
                Err(format!("the item {} is malformed", item))
            }
        })
    }
}

impl Draw {
    fn possible(&self, maxes: &Draw) -> bool {
        maxes.greens >= self.greens && maxes.blues >= self.blues && maxes.reds >= self.reds
    }
}

#[derive(Debug)]
struct Game {
    draws: Vec<Draw>,
    id: u32
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((game_id_str, draws_str)) = s.split(": ").collect_tuple() {
            let id_str = game_id_str.trim_start_matches("Game ");
            let id = id_str.parse().map_err(|e: ParseIntError| e.to_string())?;
            let draws: Result<Vec<Draw>,String> = draws_str.split("; ").map(|draw_str| draw_str.parse()).collect();
            let draws = draws?;
            Ok(Game {
                id,
                draws,
            })
        } else {
            Err(format!("The game {} is malformed", s))
        }
    }
}

impl Game {
    fn possible(&self, maxes: &Draw) -> bool {
        self.draws.iter().all(|draw| draw.possible(&maxes))
    }
    fn fewest_cubes(&self) -> Draw {
        self.draws.iter().fold(Draw{
            greens: 0,
            blues: 0,
            reds: 0,
        }, |sofar, draw| {
            Draw {
                greens: if sofar.greens > draw.greens { sofar.greens } else {draw.greens},
                blues: if sofar.blues > draw.blues { sofar.blues } else {draw.blues},
                reds: if sofar.reds > draw.reds { sofar.reds } else {draw.reds},
            }
        })
    }
    fn power(&self) -> u32 {
        let fc = self.fewest_cubes();
        fc.greens * fc.blues * fc.reds
    }

}


fn q1(input: &str) -> u32 {
    let games: Result<Vec<Game>, String> = input.lines().map(|line| line.parse::<Game>()).collect();
    if games.is_err() { // I'm worryingly golangified
        println!("Error reading: {}", games.unwrap_err());
        return 0
    }
    let games = games.unwrap();
    let possible_games = games.iter().filter(|g| g.possible(&Draw{
        reds: 12,
        greens: 13,
        blues: 14,
    }));
    let sum = possible_games.map(|g| g.id).sum();
    sum
}

fn q2(input: &str) -> u32 {
    let games: Result<Vec<Game>, String> = input.lines().map(|line| line.parse::<Game>()).collect();
    if games.is_err() { // I'm worryingly golangified
        println!("Error reading: {}", games.unwrap_err());
        return 0
    }
    let games = games.unwrap();
    games.iter().map(|g| g.power()).sum()
}