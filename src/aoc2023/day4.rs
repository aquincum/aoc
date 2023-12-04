use std::collections::HashSet;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;
use itertools::Itertools;

use crate::common::day::{Day, Question};

pub struct Day4;

impl Day for Day4 {
    fn question(&self, input: &str, question: Question) {
        let result = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        if result.is_err() {
            println!("Error: {}", result.unwrap_err());
        } else {
            println!("{}", result.unwrap());
        }
    }

    fn test_data(&self) -> String {
        return "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();
    }
}

struct Card {
    id: usize,
    winning: Vec<usize>,
    picked: Vec<usize>
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").collect_vec();
        let idpart = parts.get(0).ok_or("no :")?;
        let idpart = idpart.trim_start_matches("Card ").trim();
        let id  = idpart.parse().map_err(|e: ParseIntError| format!("in {}: {}", idpart, e.to_string()))?;
        let (winning, picked) = parts.get(1).ok_or("no : long enough")?.split(" | ").collect_tuple().ok_or("no |")?;
        let winning: Result<Vec<usize>,String> = winning.split_ascii_whitespace()
            .map(|num| num.parse().map_err(|e: ParseIntError| format!("in {}: {}", num, e.to_string())))
            .collect();
        let winning = winning?;
        let picked: Result<Vec<usize>,String> = picked.split_ascii_whitespace()
            .map(|num| num.parse().map_err(|e: ParseIntError| format!("in {}: {}", num, e.to_string())))
            .collect();
        let picked = picked?;
        Ok(Card{
            id, winning, picked
        })
    }
}

impl Card {
    fn points(&self) -> u32 {
        match self.wins() {
            0 => 0,
            x => 2u32.pow(x as u32-1),
        }
    }
    fn wins(&self) -> u32 {
        let winners: HashSet<&usize> = HashSet::from_iter(self.winning.iter());
        let winning_picked = self.picked.iter().filter(|p| winners.contains(p)).count();
        winning_picked as u32
    }
}

struct BuyAheadState {
    future: Vec<u32>,
    cards: u32
}

impl BuyAheadState {
    fn next_card(self, c: &Card) -> Self {
        let mut future_iter = self.future.into_iter();
        let how_many_of_me = future_iter.next().unwrap_or(0u32) + 1;
        let mut next_future = future_iter.collect_vec();
        let my_wins = c.wins();
        for i in 0..my_wins {
            if i >= next_future.len() as u32 {
                next_future.push(how_many_of_me);
            } else {
                next_future[i as usize] += how_many_of_me;
            }
        }
        BuyAheadState {
            future: next_future,
            cards: self.cards + how_many_of_me
        }
    }
}

fn q1(input: &str) -> Result<u32, String> {
    let cards: Result<Vec<Card>, String> = input.lines().map(|x| x.parse()).collect();
    let cards = cards?;
    let points = cards.iter().map(|c| c.points()).sum();
    Ok(points)
}

fn q2(input: &str) -> Result<u32, String> {
    let cards: Result<Vec<Card>, String> = input.lines().map(|x| x.parse()).collect();
    let cards = cards?;
    let finalstate = cards.iter().fold(BuyAheadState{
        future: vec![],
        cards: 0,
    }, |state, c| state.next_card(c));
    Ok(finalstate.cards)
}
