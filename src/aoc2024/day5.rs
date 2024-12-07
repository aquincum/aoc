use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day5;

impl Day for Day5 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string()
    }
}

struct Rule(usize, usize);

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s
            .split("|")
            .collect_tuple()
            .ok_or(format!("no two numbers here in {}", s))?;
        let n1 = s1.parse().map_err(|e: ParseIntError| e.to_string())?;
        let n2 = s2.parse().map_err(|e: ParseIntError| e.to_string())?;
        Ok(Rule(n1, n2))
    }
}

struct Book(Vec<usize>);

impl Clone for Book {
    fn clone(&self) -> Self {
        Book(self.0.clone())
    }
}

impl FromStr for Book {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let me: Result<Vec<usize>, String> = s
            .split(",")
            .map(|s| s.parse().map_err(|e: ParseIntError| e.to_string()))
            .collect();
        let me = me?;
        Ok(Book(me))
    }
}

struct RuleBook(HashMap<usize, HashSet<usize>>);

impl RuleBook {
    fn new(vec: Vec<Rule>) -> Self {
        let hm = vec
            .into_iter()
            .map(|r| (r.0, r.1))
            .fold(HashMap::new(), |mut hm, (bef, af)| {
                if !hm.contains_key(&bef) {
                    hm.insert(bef, HashSet::from([af]));
                } else {
                    hm.get_mut(&bef).unwrap().insert(af);
                }
                hm
            });
        RuleBook(hm)
    }
    fn order(&self, bef: &usize, af: &usize) -> Ordering {
        if bef == af {
            Ordering::Equal
        } else if let Some(afs) = self.0.get(&bef) {
            if afs.contains(af) {
                Ordering::Less
            } else {
                // every pair is there..
                Ordering::Greater
            }
        } else {
            // every pair is there..
            Ordering::Greater
        }
    }
    fn passes(&self, book: &Book) -> bool {
        //ugh
        println!("\n===\n");
        for bef in 0..(book.0.len() - 1) {
            for af in bef + 1..book.0.len() {
                let ord = self.order(&book.0[bef], &book.0[af]);
                // println!("Comparing {} with {} : {:?}", book.0[bef], book.0[af], ord);
                if ord == Ordering::Greater {
                    return false; // extreme golang vibes
                }
            }
        }
        true
    }
    fn sort(&self, book: &Book) -> Book {
        let mut sorted_book = book.clone();
        sorted_book.0.sort_by(|a, b| self.order(a, b));
        sorted_book
    }
}

fn q1(input: &str) -> Result<u128, String> {
    let (rule_book, books) = parse_input(input)?;

    let sum = books
        .iter()
        .filter(|book| rule_book.passes(book))
        .map(|book| book.0[(book.0.len() - 1) / 2] as u128)
        .sum();
    Ok(sum)
}

fn q2(input: &str) -> Result<u128, String> {
    let (rule_book, books) = parse_input(input)?;

    let sum = books
        .iter()
        .filter(|book| !rule_book.passes(book))
        .map(|book| rule_book.sort(book))
        .map(|book| book.0[(book.0.len() - 1) / 2] as u128)
        .sum();
    Ok(sum)
}

fn parse_input(input: &str) -> Result<(RuleBook, Vec<Book>), String> {
    let (rules, books) = input
        .split("\n\n")
        .collect_tuple()
        .ok_or("no double line break found")?;
    let rules: Result<Vec<Rule>, String> = rules.split("\n").map(|r| r.parse()).collect();
    let rules = rules?;
    let rule_book = RuleBook::new(rules);

    let books: Result<Vec<Book>, String> = books.split("\n").map(|r| r.parse()).collect();
    let books = books?;
    Ok((rule_book, books))
}
