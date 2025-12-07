use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub struct Day23;

impl Day for Day23 {
    fn question(&self, input: &str, question: Question) {
        if question == Question::First {
            let cnt = Day23::q1(input);
            println!("{}", cnt);
        } else {
            Day23::q2(input);
        }
    }

    fn test_data(&self) -> String {
        "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
            .to_string()
    }
}

impl Day23 {
    fn q1(input: &str) -> usize {
        let connections = Day23::read_connections(input);
        let mut triplets: HashSet<(String, String, String)> = HashSet::new();
        for conn in connections.clone() {
            let combos: HashSet<(String, String, String)> = conn
                .1
                .iter()
                .combinations(2)
                .filter(|v| connections.get(v[0]).unwrap().contains(v[1]))
                .map(|v| vec![v, vec![&conn.0]].concat())
                .map(|v| {
                    v.iter()
                        .sorted()
                        .map(|s| s.clone().clone())
                        .collect_tuple()
                        .unwrap()
                })
                .collect();
            triplets = triplets.union(&combos).map(|s| s.clone()).collect();
        }
        let cnt = triplets
            .iter()
            .filter(|t| {
                t.0.chars().nth(0).unwrap() == 't'
                    || t.1.chars().nth(0).unwrap() == 't'
                    || t.2.chars().nth(0).unwrap() == 't'
            })
            .map(|trip| {
                println!("{:?}", trip);
                trip
            })
            .count();
        cnt
    }
    fn q2(input: &str) {
        let connections = Day23::read_connections(input);
        let mut size = 1usize;
        let all_computers: HashSet<_> = connections.keys().map(|x| x.clone()).collect();
        let mut friends: HashSet<_> = connections
            .iter()
            .map(|(k, v)| Computers(vec![k.clone()]))
            .collect();
        loop {
            size += 1;
            let mut new_friends = HashSet::new();
            for friendgroup in &friends {
                let friendgroup_hash = friendgroup.to_hashset();
                let candidates: HashSet<_> = all_computers.difference(&friendgroup_hash).collect();
                for candidate in candidates {
                    if connections
                        .get(candidate)
                        .unwrap()
                        .is_superset(&friendgroup_hash)
                    {
                        let new_vec = vec![friendgroup.0.clone(), vec![candidate.clone()]].concat();
                        let new_vec = new_vec.into_iter().sorted().collect();
                        new_friends.insert(Computers(new_vec));
                    }
                }
            }
            println!("Size {}: {}", size, new_friends.len());
            if new_friends.len() == 1 {
                let ordered = new_friends
                    .iter()
                    .map(|c| c.0.iter().sorted().join(","))
                    .nth(0)
                    .unwrap();
                println!("{}", ordered);
            }
            if new_friends.len() == 0 {
                return;
            }
            friends = new_friends;
        }
    }

    fn read_connections(input: &str) -> HashMap<String, HashSet<String>> {
        let mut connections = HashMap::new();
        for line in input.lines() {
            let from = line[..2].to_string();
            let to = line[3..].to_string();
            connections
                .entry(from.clone())
                .and_modify(|v: &mut HashSet<String>| {
                    v.insert(to.clone());
                })
                .or_insert(HashSet::from([to.clone()]));
            connections
                .entry(to)
                .and_modify(|v: &mut HashSet<String>| {
                    v.insert(from.clone());
                })
                .or_insert(HashSet::from([from.clone()]));
        }
        connections
    }
}

#[derive(Eq, PartialEq)]
struct Computers(Vec<String>);

impl Hash for Computers {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for s in &self.0 {
            s.hash(state);
        }
    }
}

impl Computers {
    fn to_hashset(&self) -> HashSet<String> {
        self.0.iter().map(|s| s.clone()).collect()
    }
}
