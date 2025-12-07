use crate::common::columns::Columnser;
use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day25;

impl Day for Day25 {
    fn question(&self, input: &str, question: Question) {
        let things = read_input(input);
        if let Err(e) = things {
            println!("Failed to read things: {}", e);
        } else {
            let things = things.unwrap();
            let (keys, locks) = things
                .into_iter()
                .fold((vec![], vec![]), |(keys, locks), item| {
                    if let Thing::Key(ref n) = item {
                        (vec![keys, vec![item]].concat(), locks)
                    } else {
                        (keys, vec![locks, vec![item]].concat())
                    }
                });
            let fits = keys.iter().fold(0, |acc, key| {
                acc + locks.iter().fold(0, |lock_acc, lock| {
                    if fit(key, lock) {
                        lock_acc + 1
                    } else {
                        lock_acc
                    }
                })
            });
            println!("{}", fits);
        }
    }

    fn test_data(&self) -> String {
        "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"
            .to_string()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Thing {
    Key(Vec<usize>),
    Lock(Vec<usize>),
}

impl FromStr for Thing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s.columns(5);
        match s.chars().nth(0).unwrap() {
            '.' => Ok(Thing::Key(
                cols.map(|col| {
                    col.chars()
                        .enumerate()
                        .take_while(|(_, ch)| *ch == '.')
                        .last()
                        .map(|(i, _)| 5 - i)
                        .unwrap_or(6)
                })
                .collect(),
            )),
            '#' => Ok(Thing::Lock(
                cols.map(|col| {
                    col.chars()
                        .enumerate()
                        .take_while(|(_, ch)| *ch == '#')
                        .last()
                        .map(|(i, _)| i)
                        .unwrap_or(0)
                })
                .collect(),
            )),
            _ => Err("".to_string()),
        }
    }
}

impl Display for Thing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arr = match self {
            Thing::Key(arr) => {
                write!(f, "Key ");
                arr
            }
            Thing::Lock(arr) => {
                write!(f, "Lock ");
                arr
            }
        };
        write!(f, "{}", arr.iter().map(|n| n.to_string()).join(","))
    }
}

fn read_input(input: &str) -> Result<Vec<Thing>, String> {
    input
        .split("\n\n")
        .map(|section| Thing::from_str(section))
        .collect()
}

fn fit(key: &Thing, lock: &Thing) -> bool {
    let Thing::Key(key) = key else {
        panic!("key not key")
    };
    let Thing::Lock(lock) = lock else {
        panic!("lock not lock")
    };
    key.iter().zip(lock.iter()).all(|(a, b)| *a + *b < 6)
}
