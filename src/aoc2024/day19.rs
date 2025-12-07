use crate::common::day::{Day, Question};
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;

pub struct Day19;

impl Day for Day19 {
    fn question(&self, input: &str, question: Question) {
        let availables: Vec<_> = input.lines().nth(0).unwrap().split(", ").collect_vec();
        let to_design = input.lines().skip(2).collect_vec();
        if question == Question::Second {
            q2(input, availables, to_design);
            return;
        }
        let re = Regex::new(&format!("^({})*$", availables.join("|"))).unwrap();
        let n = to_design
            .iter()
            .filter(|av| {
                // let ma = re.find(av);
                // if let Some(ma) = ma {
                //     println!("{}: {}", av, ma.as_str());
                // }
                re.is_match(av)
            })
            .count();
        println!("{}", n);
    }

    fn test_data(&self) -> String {
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            .to_string()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Possibility<'a> {
    AllOpen(usize),
    Pattern(&'a str, usize, usize),
}

fn merge_states(states: Vec<Possibility>) -> Vec<Possibility> {
    let mut merged_states = vec![];
    let groups = states
        .iter()
        .sorted_by(|a, b| match (a, b) {
            (Possibility::AllOpen(_), Possibility::AllOpen(_)) => Ordering::Equal,
            (Possibility::AllOpen(_), Possibility::Pattern(_, _, _)) => Ordering::Less,
            (Possibility::Pattern(_, _, _), Possibility::AllOpen(_)) => Ordering::Greater,
            (Possibility::Pattern(patt, pointer, _), Possibility::Pattern(patt2, pointer2, _)) => {
                match patt.cmp(patt2) {
                    Ordering::Equal => pointer.cmp(pointer2),
                    o => o,
                }
            }
        })
        .group_by(|poss| match poss {
            Possibility::AllOpen(_) => None,
            Possibility::Pattern(patt, pointer, _) => Some((patt, pointer)),
        });
    for (poss, group) in &groups {
        let sum = group
            .map(|poss| match poss {
                Possibility::AllOpen(cnt) => cnt,
                Possibility::Pattern(_, _, cnt) => cnt,
            })
            .sum();
        let merged_posses = match poss {
            None => Possibility::AllOpen(sum),
            Some((patt, pointer)) => Possibility::Pattern(patt, pointer.clone(), sum),
        };
        merged_states.push(merged_posses);
    }
    merged_states
}

fn q2(input: &str, availables: Vec<&str>, to_design: Vec<&str>) {
    let mut count = 0usize;
    for target in to_design {
        let mut state = vec![Possibility::AllOpen(1)];
        for ch in target.chars() {
            // println!("  for {}:", ch);
            let mut next_state = vec![];
            for possibility in state {
                match possibility {
                    Possibility::Pattern(patt, pointer, cnt) => {
                        // println!("    pattern {} at {}", patt, pointer);
                        if patt.chars().nth(pointer).unwrap() == ch {
                            let next_possibility = if patt.len() == pointer + 1 {
                                Possibility::AllOpen(cnt)
                            } else {
                                Possibility::Pattern(patt, pointer + 1, cnt)
                            };
                            next_state.push(next_possibility);
                        }
                    }
                    Possibility::AllOpen(cnt) => {
                        // all open
                        // println!("    all-open");
                        let mut new_possibilities = availables
                            .iter()
                            .filter_map(|av| {
                                if av.chars().nth(0).unwrap() == ch {
                                    if av.len() == 1 {
                                        Some(Possibility::AllOpen(cnt))
                                    } else {
                                        Some(Possibility::Pattern(av, 1, cnt))
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect_vec();
                        next_state.append(&mut new_possibilities);
                    }
                }
            }
            state = merge_states(next_state);
        }
        let hits = state
            .iter()
            .map(|possibility| match possibility {
                Possibility::AllOpen(cnt) => cnt.clone(),
                _ => 0,
            })
            .sum::<usize>();
        count += hits;
        println!("{}: {}", target, hits);
    }
    println!("{}", count);
}
