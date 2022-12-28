use std::num::ParseIntError;
use std::str::{FromStr, Split};

use regex::Regex;
use crate::Question;
use std::convert::identity;
use itertools::Itertools;
use std::collections::HashSet;

pub const DEMO_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

type Cost = u32;
type Quantity = u32;

struct Blueprint {
    id: usize,
    ore_ore: Cost,
    clay_ore: Cost,
    obsidian_ore: Cost,
    obsidian_clay: Cost,
    geode_ore: Cost,
    geode_obsidian: Cost,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    ore_bot: Quantity,
    clay_bot: Quantity,
    obsidian_bot: Quantity,
    geode_bot: Quantity,
    ore: Quantity,
    clay: Quantity,
    obsidian: Quantity,
    geode: Quantity
}

impl State {
    fn step(&self) -> State {
        State{
            ore_bot: self.ore_bot,
            clay_bot: self.clay_bot,
            obsidian_bot: self.obsidian_bot,
            geode_bot: self.geode_bot,
            ore: self.ore + self.ore_bot,
            clay: self.clay + self.clay_bot,
            obsidian: self.obsidian + self.obsidian_bot,
            geode: self.geode + self.geode_bot
        }
    }
    fn build_ore(&self, blueprint: &Blueprint) -> State {
        State {
            ore_bot: self.ore_bot + 1,
            clay_bot: self.clay_bot,
            obsidian_bot: self.obsidian_bot,
            geode_bot: self.geode_bot,
            ore: self.ore - blueprint.ore_ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode
        }
    }
    fn build_clay(&self, blueprint: &Blueprint) -> State {
        State {
            ore_bot: self.ore_bot,
            clay_bot: self.clay_bot + 1,
            obsidian_bot: self.obsidian_bot,
            geode_bot: self.geode_bot,
            ore: self.ore - blueprint.clay_ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode
        }
    }
    fn build_obsidian(&self, blueprint: &Blueprint) -> State {
        State {
            ore_bot: self.ore_bot,
            clay_bot: self.clay_bot,
            obsidian_bot: self.obsidian_bot + 1,
            geode_bot: self.geode_bot,
            ore: self.ore - blueprint.obsidian_ore,
            clay: self.clay - blueprint.obsidian_clay,
            obsidian: self.obsidian,
            geode: self.geode
        }
    }
    fn build_geode(&self, blueprint: &Blueprint) -> State {
        State {
            ore_bot: self.ore_bot,
            clay_bot: self.clay_bot,
            obsidian_bot: self.obsidian_bot,
            geode_bot: self.geode_bot + 1,
            ore: self.ore - blueprint.geode_ore,
            clay: self.clay,
            obsidian: self.obsidian - blueprint.geode_obsidian,
            geode: self.geode,
        }
    }
    fn greedy(&self, blueprint: &Blueprint) -> State {
        if self.obsidian >= blueprint.geode_obsidian && self.ore >= blueprint.geode_ore {
            self.build_geode(blueprint)
        } else if self.clay >= blueprint.obsidian_clay && self.ore >= blueprint.obsidian_ore {
            self.build_obsidian(blueprint)
        } else if self.ore >= blueprint.clay_ore {
            self.build_clay(blueprint)
        } else if self.ore >= blueprint.ore_ore {
            self.build_ore(blueprint)
        } else {
            self.clone()
        }
    }
}


impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore\. Each clay robot costs ([0-9]+) ore\. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay\. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian\.");
        let caps = re.map_err(|e| e.to_string())?.captures(s).ok_or("no capture")?;
        let id = caps.get(1).ok_or("No id")?.as_str().parse().map_err(|e: ParseIntError| e.to_string())?;
        let ore_ore = caps.get(2).ok_or("No ore_ore")?.as_str().parse().map_err(|e: ParseIntError| e.to_string())?;
        let clay_ore = caps.get(3).ok_or("No clay_ore")?.as_str().parse().map_err(|e: ParseIntError| e.to_string())?;
        let obsidian_ore = caps.get(4).ok_or("No obsidian_ore")?.as_str().parse().map_err(|e: ParseIntError| e.to_string())?;
        let obsidian_clay = caps.get(5).ok_or("No obsidian_clay")?.as_str().parse().map_err(|e: ParseIntError| e.to_string())?;
        let geode_ore = caps.get(6).ok_or("No geode_ore")?.as_str().parse().map_err(|e: ParseIntError| e.to_string())?;
        let geode_obsidian = caps.get(7).ok_or("No geode_obsidian")?.as_str().parse().map_err(|e: ParseIntError| e.to_string())?;
        Ok(Blueprint{
            id,
            ore_ore,
            clay_ore,
            obsidian_ore,
            obsidian_clay,
            geode_ore,
            geode_obsidian
        })
    }
}

impl Blueprint {
    fn run_blueprint(&self, rounds: usize) -> u32 {
        let rng = 1..(rounds+1);
        rng.fold(vec![State{
            ore_bot: 1,
            clay_bot: 0,
            obsidian_bot: 0,
            geode_bot: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0
        }], |acc, i| {
            println!("{}: {:?} states", i, acc.len());
            let options = acc.iter().flat_map(|s| {
                let no_build = s.step();
                let new_ore = if s.ore >= self.ore_ore {
                    Some(no_build.build_ore(self))
                } else { None };
                let new_clay = if s.ore >= self.clay_ore {
                    Some(no_build.build_clay(self))
                } else { None };
                let new_obsidian = if s.ore >= self.obsidian_ore && s.clay >= self.obsidian_clay {
                    Some(no_build.build_obsidian(self))
                } else { None };
                let new_geode = if s.ore >= self.geode_ore && s.obsidian >= self.geode_obsidian {
                    Some(no_build.build_geode(self))
                } else { None };
                [Some(no_build), new_ore, new_clay, new_obsidian, new_geode].into_iter().filter_map(|x| *x).collect::<Vec<State>>()
            });
            let future_options: HashSet<State> = options.collect();
            println!("  UH {}", future_options.iter().filter(|s| s.geode_bot != 0).count());
            let best_geode = future_options.iter().max_by_key(|s| s.geode).map(|s| s.geode).unwrap_or(0);
            future_options.into_iter().filter_map(|s| {
                  if s.geode_bot + s.geode + 1 < best_geode {
                      return    None
                  }
                let mut greedy_st = s;
                for j in (i+1)..(rounds+1) {
                    greedy_st = greedy_st.step().greedy(self);
                }
                if greedy_st.geode < best_geode {
                    None
                } else {
                    Some(s)
                }
            }).collect()
        }).iter().map(|s| s.geode).max().unwrap_or(0) * self.id as u32
    }
}

pub fn question(input: &str){
    let blueprints: Result<Vec<Blueprint>, _> = input.lines().map(|l| l.parse()).collect();
    let blueprints = blueprints.unwrap();
    let sum: Quantity = blueprints.iter().map(|b| b.run_blueprint(24)).sum();
    println!("{}", sum);
}