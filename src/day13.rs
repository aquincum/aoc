use itertools::Itertools;
use serde::Deserialize;
use serde_json::Value;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
enum Packet {
    Num(i64),
    Array(Vec<Packet>),
}

impl TryFrom<Value> for Packet {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(n) => Ok(Packet::Num(n.as_i64().unwrap())),
            Value::Array(vals) => {
                let res: Result<Vec<_>, _> =
                    vals.into_iter().map(|v| Packet::try_from(v)).collect();
                Ok(Packet::Array(res?))
            }
            _ => Err(format!("invalid value {:?}", value)),
        }
    }
}

fn well_ordered(left: &Packet, right: &Packet) -> Ordering {
    match (left, right) {
        (Packet::Num(x), Packet::Num(y)) if x < y => Ordering::Less,
        (Packet::Num(x), Packet::Num(y)) if x > y => Ordering::Greater,
        (Packet::Num(x), Packet::Num(y)) => Ordering::Equal,
        (Packet::Num(x), Packet::Array(ys)) => well_ordered(
            &Packet::Array(vec![Packet::Num(*x)]),
            &Packet::Array(ys.to_vec()),
        ),
        (Packet::Array(ys), Packet::Num(x)) => well_ordered(
            &Packet::Array(ys.to_vec()),
            &Packet::Array(vec![Packet::Num(*x)]),
        ),
        (Packet::Array(xs), Packet::Array(ys)) if xs.len() == 0 && ys.len() == 0 => Ordering::Equal,
        (Packet::Array(xs), Packet::Array(ys)) if xs.len() == 0 => Ordering::Less,
        (Packet::Array(xs), Packet::Array(ys)) if ys.len() == 0 => Ordering::Greater,
        (Packet::Array(xs), Packet::Array(ys)) => {
            let first_res = well_ordered(&xs[0], &ys[0]);
            if first_res != Ordering::Equal {
                return first_res;
            }
            let first_leftover = xs.into_iter().skip(1).map(|p| p.clone()).collect_vec();
            let second_leftover = ys.into_iter().skip(1).map(|p| p.clone()).collect_vec();
            well_ordered(
                &Packet::Array(first_leftover),
                &Packet::Array(second_leftover),
            )
        }
    }
}

pub fn question(input: &str) {
    let pairs: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|pair| {
            let tuple = pair
                .lines()
                .filter(|line| line.len() > 0)
                .map(|line| {
                    println!("{}", line);
                    let v: Value = serde_json::from_str(line).unwrap();
                    Packet::try_from(v).unwrap()
                })
                .collect_tuple()
                .unwrap();
            tuple
        })
        .collect_vec();

    let results = pairs.iter().map(|p| well_ordered(&p.0, &p.1)).collect_vec();
    println!("{:?}", results);
    println!(
        "{}",
        results
            .iter()
            .enumerate()
            .filter(|(i, o)| **o == Ordering::Less)
            .map(|(n, _)| n + 1)
            .sum::<usize>()
    );

    let mut all_packets = pairs.iter().fold(Vec::new(), |mut acc, (p1, p2)| {
        acc.push(p1.clone());
        acc.push(p2.clone());
        acc
    });
    let divider_1 = Packet::Array(vec![Packet::Array(vec![Packet::Num(2)])]);
    let divider_2 = Packet::Array(vec![Packet::Array(vec![Packet::Num(6)])]);
    all_packets.push(divider_1.clone());
    all_packets.push(divider_2.clone());
    all_packets.sort_by(|v1, v2| well_ordered(v1, v2));
    let (idx_1, _) = all_packets
        .iter()
        .find_position(|&p| p == &divider_1)
        .unwrap();
    let (idx_2, _) = all_packets
        .iter()
        .find_position(|&p| p == &divider_2)
        .unwrap();
    println!("Q2: {}", (idx_1 + 1) * (idx_2 + 1));
}
