use crate::common::day::{Day, Question};
use std::str::FromStr;

pub struct Day2;

impl Day for Day2 {
    fn question(&self, input: &str, question: Question) {
        let result = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{}", result);
    }

    fn test_data(&self) -> String {
        return "A Y
B X
C Z"
        .to_string();
    }
}

#[derive(PartialEq, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
enum RPSResult {
    Win,
    Draw,
    Loss,
}
impl RPS {
    fn score(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
    fn play(&self, enemy: &RPS) -> RPSResult {
        match (self, enemy) {
            (x, y) if x == y => RPSResult::Draw,
            (RPS::Paper, RPS::Rock) => RPSResult::Win,
            (RPS::Scissors, RPS::Paper) => RPSResult::Win,
            (RPS::Rock, RPS::Scissors) => RPSResult::Win,
            _ => RPSResult::Loss,
        }
    }
}
impl FromStr for RPS {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(format!("{} is not a valid input", s)),
        }
    }
}
impl RPSResult {
    fn score(&self) -> i32 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Draw => 3,
            RPSResult::Loss => 0,
        }
    }
    fn select_play(&self, enemy: &RPS) -> RPS {
        match (self, enemy) {
            (RPSResult::Draw, x) => x.clone(),
            (RPSResult::Loss, RPS::Rock) => RPS::Scissors,
            (RPSResult::Loss, RPS::Scissors) => RPS::Paper,
            (RPSResult::Loss, RPS::Paper) => RPS::Rock,
            (RPSResult::Win, RPS::Rock) => RPS::Paper,
            (RPSResult::Win, RPS::Paper) => RPS::Scissors,
            (RPSResult::Win, RPS::Scissors) => RPS::Rock,
        }
    }
}
impl FromStr for RPSResult {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RPSResult::Loss),
            "Y" => Ok(RPSResult::Draw),
            "Z" => Ok(RPSResult::Win),
            _ => Err(format!("{} is not a valid result input", s)),
        }
    }
}

pub fn q1(input: &str) -> i32 {
    let plays = input.lines().map(|l| {
        let pair: Vec<RPS> = l.split(" ").map(|s| s.parse::<RPS>().unwrap()).collect();
        let battle_score = pair[1].play(&pair[0]).score();
        let own_score = pair[1].score();
        battle_score + own_score
    });
    plays.sum()
}

pub fn q2(input: &str) -> i32 {
    let plays = input.lines().map(|l| {
        let str_pair: Vec<&str> = l.split(" ").collect();
        if str_pair.len() != 2 {
            panic!("weird input: {}", l)
        }
        let enemy: RPS = str_pair[0].parse().unwrap();
        let my_goal: RPSResult = str_pair[1].parse().unwrap();
        let my_play = my_goal.select_play(&enemy);
        my_goal.score() + my_play.score()
    });
    plays.sum()
}
