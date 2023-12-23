use crate::common::day::{Day, Question};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;
use std::str::FromStr;

pub struct Day20;

impl Day for Day20 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a, rx"
            .to_string()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum ModuleType {
    Plain,
    Broadcaster,
    Flipper(bool),
    Conjunction(HashMap<String, PingLevel>),
    Button,
    Counter(usize),
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    links: Vec<String>,
}

impl FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([&%]?)([a-z]+) -> (.*)$").map_err(|e| e.to_string())?;
        let caps = re
            .captures(s)
            .ok_or(format!("No capture for module {}", s))?;
        let type_marker = caps
            .get(1)
            .ok_or("no type marker capture".to_string())?
            .as_str();
        let name = caps.get(2).ok_or("no name".to_string())?.as_str();
        let links = caps.get(3).ok_or("no links".to_string())?.as_str();

        let module_type = match type_marker {
            "%" => ModuleType::Flipper(false),
            "&" => ModuleType::Conjunction(HashMap::new()),
            "" => ModuleType::Plain,
            _ => Err(format!("Unknown module type: {}", type_marker))?,
        };
        let links = links.split(", ").map(|s| s.to_string()).collect_vec();
        Ok(Module {
            name: name.to_string(),
            module_type: if name == "broadcaster" {
                ModuleType::Broadcaster
            } else {
                module_type
            },
            links,
        })
    }
}

impl ModuleType {
    fn register_links(&mut self, name: &str, rev_map: &HashMap<String, Vec<String>>) {
        match self {
            ModuleType::Conjunction(linkmap) => {
                for link in rev_map.get(name).unwrap_or(&vec![]) {
                    linkmap.insert(link.clone(), PingLevel::Low);
                }
            }
            _ => {}
        }
    }
}

impl Module {
    fn process_ping(&mut self, level: PingLevel, from: String) -> Vec<Ping> {
        match &mut self.module_type {
            &mut ModuleType::Counter(n) => {
                if level == PingLevel::Low {
                    self.module_type = ModuleType::Counter(n + 1);
                }
                self.links
                    .iter()
                    .map(|l| Ping {
                        target: l.clone(),
                        origin: self.name.clone(),
                        level,
                    })
                    .collect_vec()
            }
            &mut ModuleType::Flipper(val) => match level {
                PingLevel::High => {
                    vec![]
                }
                PingLevel::Low => {
                    self.module_type = ModuleType::Flipper(!val);
                    self.links
                        .iter()
                        .map(|l| Ping {
                            target: l.clone(),
                            level: if val { PingLevel::Low } else { PingLevel::High },
                            origin: self.name.clone(),
                        })
                        .collect_vec()
                }
            },
            mut c @ ModuleType::Conjunction(_) => {
                let all_high = c.update_conjunction(level, from);
                let level = if all_high {
                    PingLevel::Low
                } else {
                    PingLevel::High
                };
                self.links
                    .iter()
                    .map(|l| Ping {
                        target: l.clone(),
                        origin: self.name.clone(),
                        level,
                    })
                    .collect_vec()
            }
            _ => self
                .links
                .iter()
                .map(|l| Ping {
                    target: l.clone(),
                    level,
                    origin: self.name.clone(),
                })
                .collect_vec(),
        }
    }
}

impl ModuleType {
    fn update_conjunction(&mut self, level: PingLevel, from: String) -> bool {
        match self {
            ModuleType::Conjunction(c) => {
                c.insert(from, level);
                let all_high = c.values().all(|v| v == &PingLevel::High);
                all_high
            }
            _ => false,
        }
    }
}

type Machine = HashMap<String, Module>;

fn read_machine(input: &str) -> Result<Machine, String> {
    let modules = input
        .lines()
        .map(|l| l.parse::<Module>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut machine = modules
        .into_iter()
        .map(|m| (m.name.clone(), m))
        .collect::<HashMap<_, _>>();

    let rev_map: HashMap<String, Vec<String>> = machine
        .iter()
        .map(|(k, v)| v.links.iter().map(|end| (end, k.clone())).collect_vec())
        .flatten()
        .fold(HashMap::new(), |mut map, l| {
            map.entry(l.0.clone())
                .and_modify(|mut x| x.push(l.1.clone()))
                .or_insert(vec![l.1.clone()]);
            map
        });
    for (_, module) in machine.iter_mut() {
        module.module_type.register_links(&module.name, &rev_map);
    }
    machine.insert(
        "button".to_string(),
        Module {
            name: "button".to_string(),
            module_type: ModuleType::Button,
            links: vec!["broadcaster".to_string()],
        },
    );
    Ok(machine)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum PingLevel {
    High,
    Low,
}

impl Display for PingLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PingLevel::High => "high",
                PingLevel::Low => "low",
            }
        )
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]

struct Ping {
    target: String,
    origin: String,
    level: PingLevel,
}
fn broadcast_ping(machine: &mut Machine) -> (u128, u128) {
    let mut backlog = VecDeque::from(vec![Ping {
        target: "broadcaster".to_string(),
        level: PingLevel::Low,
        origin: "button".to_string(),
    }]);
    let (mut highs, mut lows) = (0, 0);
    while let Some(ping) = backlog.pop_front() {
        match ping.level {
            PingLevel::High => highs += 1,
            PingLevel::Low => lows += 1,
        }
        // println!("{} -{}-> {}", ping.origin, ping.level, ping.target);
        let mut module = machine.get_mut(&ping.target);
        if module.is_none() {
            continue; // output node
        }
        let mut module = module.unwrap();
        let pings = module.process_ping(ping.level, ping.origin);
        for ping in pings.iter() {
            backlog.push_back(ping.clone());
        }
    }
    (highs, lows)
}

fn q1(input: &str) -> Result<u128, String> {
    let mut machine = read_machine(input)?;
    println!("{:?}", machine);
    let (highs, lows) = (0..1000).fold((0, 0), |(highs, lows), _| {
        let (newhighs, newlows) = broadcast_ping(&mut machine);
        (highs + newhighs, lows + newlows)
    });
    Ok(highs * lows)
}

fn q2(input: &str) -> Result<u128, String> {
    let mut machine = read_machine(input)?;
    for (name, module) in machine.iter() {
        if module.links.iter().any(|l| l == "rx") {
            println!("rx input is {}", name);
        }
        if let ModuleType::Conjunction(cmap) = &module.module_type {
            println!("Conjunction {}: {}", name, cmap.keys().join(", "));
        }
    }
    // below: after looking at my machine, just hacking into it to see what's up
    let mut cnt = 0;
    let mut cycles = HashMap::from([("sh", None), ("jf", None), ("mz", None), ("bh", None)]);
    let cycle_keys = cycles.keys().map(|c| c.clone()).collect_vec();
    for module in cycles.keys() {
        machine.entry(module.to_string()).and_modify(|m| {
            m.module_type = ModuleType::Counter(0);
        });
    }
    loop {
        cnt += 1;
        broadcast_ping(&mut machine);
        for module_name in &cycle_keys {
            let module = machine.get(*module_name).unwrap();
            if let ModuleType::Counter(n) = module.module_type {
                cycles.entry(module_name).and_modify(|mut value| {
                    if value.is_some() {
                        let x: (usize, usize) = value.unwrap();
                        if n > x.1 {
                            println!(
                                "New ping for {}: cycle {} -- distance {}",
                                module_name,
                                cnt,
                                cnt - x.0
                            );
                            *value = Some((cnt, n));
                        }
                    } else {
                        if n > 0 {
                            println!("First ping for {}: cycle {}", module_name, cnt);
                            *value = Some((cnt, n));
                        }
                    }
                });
            }
        }
        if cnt > 40000 {
            break;
        }
    }

    Ok(3)
}
fn q2_brute_force(input: &str) -> Result<u128, String> {
    // needs years to run
    let mut machine = read_machine(input)?;
    println!("{:?}", machine);
    machine.insert(
        "rx".to_string(),
        Module {
            name: "rx".to_string(),
            module_type: ModuleType::Counter(0),
            links: vec![],
        },
    );
    let mut cnt = 0;
    loop {
        cnt += 1;
        broadcast_ping(&mut machine);
        let rx = machine.get("rx").unwrap();
        if let ModuleType::Counter(n) = rx.module_type {
            if n > 0 {
                break;
            }
        }
        if cnt % 10_000 == 0 {
            print!(".");
            io::stdout().flush().unwrap();
            if cnt % 1_000_000 == 0 {
                println!("{} million", cnt / 1_000_000);
            }
        }
    }
    Ok(cnt)
}
