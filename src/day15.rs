use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
//
// enum Spot {
//     Beacon,
//     Sensor,
//     Air(bool), // reachable
// }
//
// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// impl Display for Spot {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let symbol = match self {
//             Spot::Beacon => "B",
//             Spot::Sensor => "S",
//             Spot::Air(true) => "#",
//             Spot::Air(false) => ".",
//         };
//         write!(f, "{}", symbol)
//     }
// }
//
// impl Default for Spot {
//     fn default() -> Self {
//         Spot::Air(false)
//     }
// }
//
//
//

fn manhattan(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

struct Sensor {
    x: i64,
    y: i64,
    bx: i64,
    by: i64,
    range: i64,
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();
        let caps = re.captures(s).ok_or("No capture")?;
        let x = caps
            .get(1)
            .ok_or("No x")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| format!("x not a number {}", e))?;
        let y = caps
            .get(2)
            .ok_or("No y")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| "y not a number")?;
        let bx = caps
            .get(3)
            .ok_or("No bx")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| "bx not a number")?;
        let by = caps
            .get(4)
            .ok_or("No by")?
            .as_str()
            .parse()
            .map_err(|e: ParseIntError| "by not a number")?;
        Ok(Sensor {
            x,
            y,
            bx,
            by,
            range: manhattan(x, y, bx, by),
        })
    }
}

impl Sensor {
    fn line_coverage(&self, target_y: i64) -> Option<(i64, i64)> {
        let height_diff = (self.y - target_y).abs();
        let side_width = (self.range - height_diff);
        if side_width <= 0 {
            return None;
        }
        if self.by == target_y {
            if side_width == 1 {
                None
            } else if self.bx == self.x - side_width {
                Some((self.x - side_width + 1, self.x + side_width))
            } else {
                Some((self.x - side_width, self.x + side_width - 1))
            }
        } else {
            Some((self.x - side_width, self.x + side_width))
        }
    }
}

pub fn q2(input: &str) {
    let sensors: Result<Vec<Sensor>, _> = input.split("\n").map(|l| l.parse()).collect();
    let sensors = sensors.unwrap();

    let beacons: HashSet<(i64, i64)> = sensors.iter().map(|s| (s.bx, s.by)).collect();

    const search_space: i64 = 4000000;
    for y in 0..search_space {
        let mut spans = sensors
            .iter()
            .filter_map(|s| s.line_coverage(y))
            .collect_vec();

        // no brain
        spans.sort_by_key(|sp| sp.0);
        let mut curr_end = None;
        for sp in spans {
            if curr_end.is_none() {
                curr_end = Some(sp.1);
            } else {
                let ce = curr_end.unwrap();
                if ce + 1 < sp.0 {
                    println!("{}: {}-{}", y, ce + 1, sp.0 - 1);
                    if !beacons.contains(&(sp.0 - 1, y)) {
                        println!("^^ FOUND: {}", (sp.0 - 1) * 4000000 + y);
                    }
                    curr_end = Some(sp.1);
                } else if ce < sp.1 {
                    curr_end = Some(sp.1);
                }
            }
        }
    }
}

pub fn q1(input: &str) {
    let sensors: Result<Vec<Sensor>, _> = input.split("\n").map(|l| l.parse()).collect();
    let sensors = sensors.unwrap();
    let target_y = 2000000;
    // let target_y = 10;
    let mut spans = sensors
        .iter()
        .filter_map(|s| s.line_coverage(target_y))
        .collect_vec();

    // no brain
    spans.sort_by_key(|sp| sp.0);
    #[derive(Copy, Clone, Debug)]
    struct Accum {
        current_span_begins: Option<i64>,
        current_span_ends: Option<i64>,
        sum: i64,
    };
    let result = spans.iter().fold(
        Accum {
            sum: 0,
            current_span_begins: None,
            current_span_ends: None,
        },
        |acc, span| {
            println!(
                "{}-{} current state: {:?}-{:?} {},",
                span.0, span.1, acc.current_span_begins, acc.current_span_ends, acc.sum
            );
            if acc.current_span_begins == None {
                let a = Accum {
                    current_span_begins: Some(span.0),
                    current_span_ends: Some(span.1),
                    sum: acc.sum,
                };
                println!("{:?}", a);
                a
            } else if span.0 > acc.current_span_ends.unwrap() {
                Accum {
                    current_span_begins: Some(span.0),
                    current_span_ends: Some(span.1),
                    sum: acc.sum
                        + (acc.current_span_ends.unwrap() - acc.current_span_begins.unwrap() + 1),
                }
            } else if span.1 < acc.current_span_ends.unwrap() {
                acc
            } else {
                Accum {
                    current_span_begins: acc.current_span_begins,
                    current_span_ends: Some(span.1),
                    sum: acc.sum,
                }
            }
        },
    );
    let sum =
        result.sum + (result.current_span_ends.unwrap() - result.current_span_begins.unwrap() + 1);
    println!("{}", sum);
}
