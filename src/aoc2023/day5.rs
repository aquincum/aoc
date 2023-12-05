use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day5;

impl Day for Day5 {
    fn question(&self, input: &str, question: Question) {
        let result = match question {
            Question::First => q1(input),
            Question::Second => q2(input), // WIP :(
        };
        if result.is_err() {
            println!("Error: {}", result.unwrap_err());
        } else {
            println!("{}", result.unwrap());
        }
    }

    fn test_data(&self) -> String {
        return "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .to_string();
    }
}

#[derive(Copy, Clone)]
struct Span {
    from: u128,
    to: u128,
    transformed: bool,
}

impl Span {
    fn clear(self) -> Span {
        Span {
            from: self.from,
            to: self.to,
            transformed: false,
        }
    }
}

struct Mapping {
    source_from: u128,
    source_to: u128,
    destination_from: u128,
}

impl FromStr for Mapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_from, source_from, cnt) = s
            .split_ascii_whitespace()
            .map(|s| s.parse().map_err(|e: ParseIntError| e.to_string()))
            .collect_tuple()
            .ok_or(format!("Cannot split {}", s))?;
        let source_from = source_from?;
        let source_to = source_from + cnt? - 1;
        let destination_from = destination_from?;
        Ok(Mapping {
            source_from,
            source_to,
            destination_from,
        })
    }
}

impl Mapping {
    fn transform(&self, x: &u128) -> u128 {
        if *x < self.source_from || *x > self.source_to {
            *x
        } else {
            *x - self.source_from + self.destination_from
        }
    }
    fn transform_span(&self, span: &Span) -> Vec<Span> {
        if span.transformed {
            vec![span.clone()]
        } else if span.to < self.source_from || span.from > self.source_to {
            vec![span.clone()]
        } else if span.from < self.source_from && span.to <= self.source_to {
            vec![
                Span {
                    from: span.from,
                    to: self.source_from - 1,
                    transformed: false,
                },
                Span {
                    from: self.destination_from,
                    to: self.transform(&span.to),
                    transformed: true,
                },
            ]
        } else if span.from >= self.source_from && span.to > self.source_to {
            vec![
                Span {
                    from: self.transform(&span.from),
                    to: self.transform(&self.source_to),
                    transformed: true,
                },
                Span {
                    from: self.source_to + 1,
                    to: span.to,
                    transformed: false,
                },
            ]
        } else if span.from < self.source_from && span.to > self.source_to {
            vec![
                Span {
                    from: span.from,
                    to: self.source_from - 1,
                    transformed: false,
                },
                Span {
                    from: self.destination_from,
                    to: self.transform(&self.source_to),
                    transformed: true,
                },
                Span {
                    from: self.source_to + 1,
                    to: span.to,
                    transformed: false,
                },
            ]
        } else {
            vec![Span {
                from: self.transform(&span.from),
                to: self.transform(&span.to),
                transformed: true,
            }]
        }
    }
}

struct Map {
    source_type: String, // it seems these don't quite matter
    destination_type: String,
    mappings: Vec<Mapping>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut l = s.lines();
        let def_line = l.next().ok_or("No line?")?;
        let def_line = def_line.trim_end_matches(" map:");
        let (source_type, destination_type) = def_line
            .split("-to-")
            .collect_tuple()
            .ok_or(format!("Can't split {}", def_line))?;
        let mappings: Result<Vec<Mapping>, String> =
            l.map(|line| line.parse::<Mapping>()).collect();
        let mappings = mappings?;
        Ok(Map {
            source_type: source_type.to_string(),
            destination_type: destination_type.to_string(),
            mappings,
        })
    }
}

impl Map {
    fn transform(&self, x: &u128) -> u128 {
        let mapping = self
            .mappings
            .iter()
            .find(|m| m.source_from <= *x && *x <= m.source_to);
        match mapping {
            None => *x,
            Some(m) => m.transform(x),
        }
    }
    fn transform_span(&self, span: &Span) -> Vec<Span> {
        self.mappings
            .iter()
            .fold(vec![span.clone()], |spans, mapping| {
                spans
                    .iter()
                    .map(|span| mapping.transform_span(span))
                    .flatten()
                    .collect_vec()
            })
            .iter()
            .map(|span| span.clear())
            .collect_vec()
    }
}

fn read_all(input: &str) -> Result<(Vec<u128>, Vec<Map>), String> {
    let mut paragraphs = input.split("\n\n");
    let seeds: Result<Vec<u128>, String> = paragraphs
        .next()
        .ok_or("no content")?
        .trim_start_matches("seeds: ")
        .split_ascii_whitespace()
        .map(|s| {
            s.parse()
                .map_err(|e: ParseIntError| format!("Can't parse {}", s))
        })
        .collect();
    let seeds = seeds?;
    let maps: Result<Vec<Map>, String> = paragraphs.map(|s| s.parse()).collect();
    let maps = maps?;
    Ok((seeds, maps))
}

fn q1(input: &str) -> Result<u128, String> {
    let (seeds, maps) = read_all(input)?;
    let final_seeds = maps.iter().fold(seeds, |seeds, map| {
        seeds.iter().map(|seed| map.transform(seed)).collect_vec()
    });
    Ok(final_seeds.iter().min().ok_or("no minimum?")?.clone())
}

fn q2(input: &str) -> Result<u128, String> {
    let (seeds, maps) = read_all(input)?;
    let seed_pairs = seeds
        .iter()
        .batching(|it| match it.next() {
            None => None,
            Some(x) => match it.next() {
                None => None,
                Some(y) => Some((x, y)),
            },
        })
        .collect_vec();
    let spans = seed_pairs
        .into_iter()
        .map(|(from, cnt)| Span {
            from: *from,
            to: *from + cnt - 1,
            transformed: false,
        })
        .collect_vec();
    let final_spans = maps.iter().fold(spans, |spans, map| {
        println!(
            "Spans before {}-to-{}:\n{}\n",
            map.source_type,
            map.destination_type,
            spans
                .iter()
                .map(|sp| format!("FROM {} TO {}", sp.from, sp.to))
                .join("\n")
        );
        spans
            .iter()
            .map(|span| map.transform_span(span))
            .flatten()
            .collect_vec()
    });
    let min = final_spans.iter().min_by_key(|span| span.from).unwrap();
    Ok(min.from)
}
