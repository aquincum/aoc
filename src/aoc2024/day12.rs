// ugly and unmaintainable but works ':)

use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day12;

impl Day for Day12 {
    fn question(&self, input: &str, question: Question) {
        println!("{:?}", q(input));
    }

    fn test_data(&self) -> String {
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            .to_string()
    }
}

struct Region {
    name: char,
    coords: HashSet<(usize, usize)>,
}

impl Region {
    fn area(&self) -> u128 {
        self.coords.len() as u128
    }
    fn perimeter(&self) -> u128 {
        let mut perim = 0;
        for (i, j) in &self.coords {
            if i.clone() == 0 || !self.coords.contains(&(i.clone() - 1, j.clone())) {
                perim += 1
            }
            if j.clone() == 0 || !self.coords.contains(&(i.clone(), j.clone() - 1)) {
                perim += 1
            }
            if !self.coords.contains(&(i.clone() + 1, j.clone())) {
                perim += 1
            }
            if !self.coords.contains(&(i.clone(), j.clone() + 1)) {
                perim += 1
            }
        }
        perim
    }
    fn sides(&self) -> u128 {
        let mut perim = 0;
        let mut sidemaps: HashMap<(usize, usize), u8> = HashMap::new();
        let sorted_coords = self.coords.iter().sorted().collect_vec();
        for (i, j) in sorted_coords {
            if i.clone() == 0 || !self.coords.contains(&(i.clone() - 1, j.clone())) {
                sidemaps
                    .entry((i.clone(), j.clone()))
                    .and_modify(|mut x| *x |= 1)
                    .or_insert(1);
                if j.clone() == 0
                    || sidemaps.get(&(i.clone(), j.clone() - 1)).unwrap_or(&0) & 1 == 0
                {
                    perim += 1
                }
            }
            if j.clone() == 0 || !self.coords.contains(&(i.clone(), j.clone() - 1)) {
                sidemaps
                    .entry((i.clone(), j.clone()))
                    .and_modify(|mut x| *x |= 2)
                    .or_insert(2);
                if i.clone() == 0
                    || sidemaps.get(&(i.clone() - 1, j.clone())).unwrap_or(&0) & 2 == 0
                {
                    perim += 1
                }
            }
            if !self.coords.contains(&(i.clone() + 1, j.clone())) {
                sidemaps
                    .entry((i.clone(), j.clone()))
                    .and_modify(|mut x| *x |= 4)
                    .or_insert(4);
                if j.clone() == 0
                    || sidemaps.get(&(i.clone(), j.clone() - 1)).unwrap_or(&0) & 4 == 0
                {
                    perim += 1
                }
            }
            if !self.coords.contains(&(i.clone(), j.clone() + 1)) {
                sidemaps
                    .entry((i.clone(), j.clone()))
                    .and_modify(|mut x| *x |= 8)
                    .or_insert(8);
                if i.clone() == 0
                    || sidemaps.get(&(i.clone() - 1, j.clone())).unwrap_or(&0) & 8 == 0
                {
                    perim += 1
                }
            }
        }
        perim
    }
}

fn q(input: &str) -> (u128, u128) {
    let char_map = input
        .lines()
        .map(|line| line.chars().map(|ch| ch).collect_vec())
        .collect_vec();
    let mut regions = Vec::new();
    let mut visited = HashSet::new();
    for (i, row) in char_map.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if visited.contains(&(i.clone(), j.clone())) {
                continue;
            }
            let mut new_region = Region {
                name: ch.clone(),
                coords: find_region(&char_map, (&i, &j), ch, &mut visited),
            };

            regions.push(new_region);
        }
    }

    let mut res = 0;
    let mut res2 = 0;
    for r in regions {
        println!(
            "{}: {:?} per {} are {} side {}",
            r.name,
            r.coords
                .iter()
                .map(|(x, y)| format!("({},{})", x, y))
                .join(", "),
            r.perimeter(),
            r.area(),
            r.sides(),
        );
        res += r.perimeter() * r.area();
        res2 += r.area() * r.sides();
    }

    (res, res2)
}

fn find_region(
    map: &Vec<Vec<char>>,
    coord: (&usize, &usize),
    ch: &char,
    visited: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    visited.insert((coord.0.clone(), coord.1.clone()));
    if coord.0.clone() != 0
        && map[coord.0.clone() - 1][coord.1.clone()] == ch.clone()
        && !visited.contains(&(coord.0.clone() - 1, coord.1.clone()))
    {
        //ups
        result = result
            .union(&find_region(
                map,
                (&(coord.0.clone() - 1), coord.1),
                ch,
                visited,
            ))
            .map(|x| *x)
            .collect();
    }
    if coord.0.clone() != map.len() - 1
        && map[coord.0.clone() + 1][coord.1.clone()] == ch.clone()
        && !visited.contains(&(coord.0.clone() + 1, coord.1.clone()))
    {
        //downs
        result = result
            .union(&find_region(map, (&(coord.0 + 1), coord.1), ch, visited))
            .map(|x| *x)
            .collect();
    }
    if coord.1.clone() != 0
        && map[coord.0.clone()][coord.1.clone() - 1] == ch.clone()
        && !visited.contains(&(coord.0.clone(), coord.1.clone() - 1))
    {
        //lefts
        result = result
            .union(&find_region(map, (coord.0, &(coord.1 - 1)), ch, visited))
            .map(|x| *x)
            .collect();
    }
    if coord.1.clone() != map[0].len() - 1
        && map[coord.0.clone()][coord.1.clone() + 1] == ch.clone()
        && !visited.contains(&(coord.0.clone(), coord.1.clone() + 1))
    {
        //ups
        result = result
            .union(&find_region(map, (&(coord.0), &(coord.1 + 1)), ch, visited))
            .map(|x| *x)
            .collect();
    }

    result.insert((coord.0.clone(), coord.1.clone()));
    result
}
