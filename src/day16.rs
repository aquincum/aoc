use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_link;
use std::num::ParseIntError;
use std::str::FromStr;
use crate::common::day::{Day, Question};

pub struct Day16;
impl Day for Day16 {
    fn question(&self, input: &str, question: Question) {
        question_fail1(input);
    }

    fn test_data(&self) -> String {
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II".to_string()
    }
}

type ValveId = String;
type ValveMap = HashMap<ValveId, Valve>;

struct Valve {
    id: ValveId,
    tunnels: Vec<ValveId>,
    flow: u32,
}

impl FromStr for Valve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{}", s);
        let re =
            Regex::new(r"Valve ([A-Z]*) has flow rate=([0-9]*); tunnels? leads? to valves? (.*)")
                .unwrap();
        let caps = re.captures(s).ok_or("No capture")?;
        let id = caps.get(1).ok_or("No ID")?.as_str().to_string();
        let flow = caps
            .get(2)
            .ok_or("No valve")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let tunnels = caps.get(3).ok_or("No tunnels")?.as_str();
        let tunnels = tunnels
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Ok(Valve {
            id: id.clone(),
            flow,
            tunnels: [tunnels, vec![id]].concat(),
        })
    }
}

struct ValveState {
    open: Vec<ValveId>,
    releases: Vec<u32>,
    debug: Vec<String>,
}

impl ValveState {
    fn sum(&self) -> u32 {
        self.releases.iter().sum()
    }
}

impl Valve {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

fn read_valve_map(input: &str) -> ValveMap {
    input
        .split("\n")
        .map(|l| l.parse::<Valve>().unwrap())
        .map(|v| (v.get_id(), v))
        .collect()
}

pub fn question(input: &str) {}

pub fn question_fail1(input: &str) {
    let valve_map = read_valve_map(input);
    let mut state = HashMap::<&ValveId, ValveState>::new();
    for vid in valve_map.keys() {
        state.insert(
            vid,
            ValveState {
                open: Vec::new(),
                releases: Vec::new(),
                debug: Vec::new(),
            },
        );
    }
    //1905..2007
    for minute in 0..30 {
        let mut new_state = HashMap::<&ValveId, ValveState>::new();
        for vid in valve_map.keys() {
            let my_valve = valve_map.get(vid).unwrap();
            let best_tunnel_id = my_valve
                .tunnels
                .iter()
                .max_by_key(|t| state.get(t).unwrap().sum())
                .unwrap();
            let best_tunnel = state.get(best_tunnel_id).unwrap();
            let current_place = state.get(vid).unwrap();
            let am_i_opened = current_place.open.iter().any(|id| id == vid);
            if minute == 0
                || my_valve.flow == 0
                || am_i_opened
                || best_tunnel.sum() > current_place.sum() + my_valve.flow
            {
                let msg = format!(
                    "==Minute {}==\nI am at {}. Next round we'll move to {}.",
                    30 - minute,
                    vid,
                    best_tunnel_id
                );
                new_state.insert(
                    vid,
                    ValveState {
                        open: best_tunnel.open.clone(),
                        releases: [vec![0], best_tunnel.releases.to_owned()].concat(),
                        // sum: best_tunnel.sum + best_tunnel.curr_rate,
                        // curr_rate: best_tunnel.curr_rate,
                        debug: [best_tunnel.debug.to_owned(), vec![msg]].concat(),
                    },
                );
            } else {
                let msg = format!(
                    "==Minute {}==\nI am at {} and I open myself",
                    30 - minute,
                    vid
                );
                new_state.insert(
                    vid,
                    ValveState {
                        open: [current_place.open.to_owned(), vec![vid.to_string()]].concat(),
                        releases: [
                            vec![0],
                            current_place
                                .releases
                                .iter()
                                .map(|r| r + my_valve.flow)
                                .collect_vec(),
                        ]
                        .concat(),
                        // sum: current_place.sum + my_valve.flow + current_place.curr_rate,
                        // curr_rate: current_place.curr_rate + my_valve.flow,
                        debug: [current_place.debug.to_owned(), vec![msg]].concat(),
                    },
                );
            }
            if vid == "AA" {
                println!(
                    "{:?} {:?}",
                    new_state.get(vid).unwrap().releases,
                    new_state.get(vid).unwrap().sum()
                );
                let ns = new_state.get(vid).unwrap();
                let opt =
                    ns.sum() + ((30 - ns.releases.len()) as u32) * ns.releases.last().unwrap_or(&0);
                println!("OPT: {}", opt);
                // println!("  {:?} {:?}", current_place.releases, current_place.releases.iter().map(|r| r+my_valve.flow).collect_vec())
            }
        }
        state = new_state;
    }
    let aa_state = state.get(&"AA".to_string()).unwrap();
    println!(
        "{}",
        aa_state
            .debug
            .iter()
            .zip(&aa_state.releases.iter().rev().collect_vec())
            .map(|(msg, r)| format!("{}\nReleasing {} now", msg, r))
            .join("\n\n")
    );

    println!(
        "For AA the state is open={}; sum={}",
        aa_state.open.join(","),
        aa_state.sum()
    );
}
