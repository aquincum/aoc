use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::string::ToString;

pub struct Day7;

impl Day for Day7 {
    fn question(&self, input: &str, question: Question) {
        let result = q(input, question);
        if result.is_err() {
            println!("Error: {}", result.unwrap_err());
        } else {
            println!("{}", result.unwrap());
        }
    }

    fn test_data(&self) -> String {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string()
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
enum HandValue {
    High,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

struct Hand {
    value: HandValue,
    cards: Vec<Card>,
    bid: u128,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Card(char, Question);
impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let char_order = match self.1 {
            Question::First => "23456789TJQKA",
            Question::Second => "J23456789TQKA",
        };
        let mine = char_order.chars().position(|ch| ch == self.0);
        let theirs = char_order.chars().position(|ch| ch == other.0);
        if mine.is_none() {
            None
        } else if theirs.is_none() {
            None
        } else {
            mine.unwrap().partial_cmp(&theirs.unwrap())
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            None => Ordering::Equal,
            Some(x) => x,
        }
    }
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u128, question: Question) -> Result<Self, String> {
        use HandValue::*;

        let mut cards_map = cards.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(c.0).or_insert(0) += 1;
            map
        });
        if question == Question::Second {
            // TODO let's refactor this once we had a lot of sleep
            let jokers = cards_map.get(&'J');
            if jokers.is_some() {
                let jokers = jokers.unwrap().clone();
                let mx = cards_map
                    .iter()
                    .filter(|c| c.0 != &'J')
                    .max_by(|(_, b), (_, c)| b.cmp(c));
                if mx.is_some() && jokers != 5 {
                    // wow very pyramid such imperative
                    cards_map
                        .entry(*mx.unwrap().0)
                        .and_modify(|mut n| *n += jokers);
                    cards_map.entry('J').and_modify(|mut n| *n = 0);
                }
            }
        }
        let value = match cards_map.values().max() {
            Some(5) => Ok(Five),
            Some(4) => Ok(Four),
            Some(3) => {
                if cards_map.values().any(|x| *x == 2) {
                    Ok(Full)
                } else {
                    Ok(Three)
                }
            }
            Some(2) => {
                if cards_map.values().filter(|x| **x == 2).count() == 2 {
                    Ok(TwoPair)
                } else {
                    Ok(Pair)
                }
            }
            Some(1) => Ok(High),
            Some(x) => Err(format!(
                "Impossible count: {} {}",
                cards.iter().map(|c| c.0).collect::<String>(),
                x
            )),
            None => Err("No max value?".to_string()),
        }?;
        Ok(Hand { cards, bid, value })
    }
    fn build(s: &str, question: Question) -> Result<Self, String> {
        let (cards, bid_str) = s
            .split_ascii_whitespace()
            .collect_tuple()
            .ok_or("Not two".to_string())?;
        let bid = bid_str.parse().map_err(|e: ParseIntError| e.to_string())?;
        Hand::new(
            cards.chars().map(|ch| Card(ch, question)).collect_vec(),
            bid,
            question,
        )
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards
            .iter()
            .enumerate()
            .all(|(i, ch)| other.cards[i] == *ch)
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.value.cmp(&other.value) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => {
                for (i, ch) in self.cards.iter().enumerate() {
                    if ch < &other.cards[i] {
                        return Some(Ordering::Less);
                    } else if ch > &other.cards[i] {
                        return Some(Ordering::Greater);
                    }
                }
                None
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            None => Ordering::Equal,
            Some(x) => x,
        }
    }
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let hands: Result<Vec<Hand>, String> = input
        .lines()
        .map(|line| Hand::build(line, question))
        .collect();
    let hands = hands?;
    let val = hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, h)| {
            println!(
                "Hand #{}: {} -- {:?}",
                i,
                h.cards.iter().map(|c| c.0).collect::<String>(),
                h.value
            );
            (i, h)
        })
        .map(|(i, h)| (i as u128 + 1) * h.bid)
        .sum();
    Ok(val)
}
