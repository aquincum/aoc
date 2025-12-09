use crate::common::columns::Columnser;
use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

pub struct Day8;

impl Day for Day8 {
    fn question(&self, input: &str, question: Question) {
        let points: Result<Vec<_>, String> = input.lines().map(|l| l.parse::<Point3D>()).collect();
        if let Err(e) = points {
            println!("Error: {}", e);
            return;
        }
        let points = points.unwrap();
        let pairwise_vec = (0..points.len())
            .map(|i| {
                (i + 1..points.len())
                    .map(|j| build_pairwise(&points, i, j))
                    .collect_vec()
            })
            .concat();
        let mut group_map: HashMap<usize, usize> = (0..points.len()).map(|i| (i, i)).collect();
        for (i, pair) in pairwise_vec.iter().sorted().enumerate() {
            println!("{}", pair.dist);
            let gr1 = group_map.get(&pair.i1).unwrap();
            let gr2 = group_map.get(&pair.i2).unwrap();
            let smaller = gr1.clone().min(gr2.clone());
            let bigger = gr1.clone().max(gr2.clone());
            group_map
                .iter_mut()
                .filter(|(_, v)| **v == bigger)
                .for_each(|(_, v)| {
                    *v = smaller;
                });

            if i == 9 || i == 999 {
                println!(
                    "After {}: {} circuits, {} Q1s",
                    i + 1,
                    get_count(&group_map),
                    get_q1(&group_map)
                );
            }
            if get_count(&group_map) == 1 {
                println!("Q1: {}", points[pair.i1].x * points[pair.i2].x);
                break;
            }
        }
        println!("Goodbye")
    }

    fn test_data(&self) -> String {
        "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .to_string()
    }
}

struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

impl FromStr for Point3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .ok_or(format!("bad 3d point: {}", s))?;
        Ok(Point3D { x, y, z })
    }
}
fn distance(p1: &Point3D, p2: &Point3D) -> f64 {
    let dx = p1.x as f64 - p2.x as f64;
    let dy = p1.y as f64 - p2.y as f64;
    let dz = p1.z as f64 - p2.z as f64;
    let sq = dx * dx + dy * dy + dz * dz;
    sq.sqrt()
}

#[derive(PartialEq)]
struct Pairwise {
    i1: usize,
    i2: usize,
    dist: f64,
}

impl Eq for Pairwise {}

fn build_pairwise(points: &Vec<Point3D>, i1: usize, i2: usize) -> Pairwise {
    Pairwise {
        i1,
        i2,
        dist: distance(&points[i1], &points[i2]),
    }
}

impl PartialOrd for Pairwise {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl Ord for Pairwise {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.total_cmp(&self.dist)
    }
}

fn get_count<K, V>(m: &HashMap<K, V>) -> usize
where
    V: Eq + Hash,
{
    let group_set: HashSet<&V> = m.values().collect();
    group_set.len()
}

fn get_q1<K, V>(m: &HashMap<K, V>) -> usize
where
    V: Eq + Hash + Debug,
{
    let group_counts: HashMap<&V, usize> = m.values().counts_by(|x| x);
    println!(
        "{:?}",
        group_counts.values().sorted().rev().take(3).collect_vec()
    );
    group_counts.values().sorted().rev().take(3).product()
}
