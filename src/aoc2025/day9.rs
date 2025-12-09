use crate::common::columns::Columnser;
use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

pub struct Day9;

impl Day for Day9 {
    fn question(&self, input: &str, question: Question) {
        let points: Result<Vec<_>, String> = input.lines().map(|l| l.parse::<Point2D>()).collect();
        if let Err(e) = points {
            println!("Error: {}", e);
            return;
        }
        let points = points.unwrap();
        let pairwise_vec = (0..points.len())
            .map(|i| {
                (i + 1..points.len())
                    .filter_map(|j| {
                        if !points[i].added && !points[j].added {
                            Some(build_pairwise(&points, i, j))
                        } else {
                            None
                        }
                    })
                    .collect_vec()
            })
            .concat();
        let q1 = pairwise_vec.iter().sorted().rev().nth(0).unwrap();
        println!("q1: {}", q1.area);
        for pw in pairwise_vec.iter().sorted().rev() {
            let concave = points.iter().enumerate().any(|(i, p)| {
                i != pw.i1 && i != pw.i2 && p.between(&points[pw.i1], &points[pw.i2])
            });
            if !concave {
                println!("q2: {}", pw.area); // 4549093344 too high 1381772324 too low
                println!(
                    "{},{} {},{}",
                    points[pw.i1].x, points[pw.i1].y, points[pw.i2].x, points[pw.i2].y
                );
                paint(&points, &points[pw.i1], &points[pw.i2]);
                break;
            }
        }
    }

    fn test_data(&self) -> String {
        "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        .to_string()
    }
}

struct Point2D {
    x: u32,
    y: u32,
    added: bool,
}

impl Point2D {
    fn between(&self, p1: &Point2D, p2: &Point2D) -> bool {
        let x1 = p1.x;
        let y1 = p1.y;
        let x2 = p2.x;
        let y2 = p2.y;
        let x = self.x;
        let y = self.y;
        ((x1 < x && x < x2) || (x2 < x && x < x1)) && ((y1 < y && y < y2) || (y2 < y && y < y1))
    }
}

impl FromStr for Point2D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let added = s.chars().nth(0).unwrap() == '!';
        let s = if added { &s[1..] } else { s };
        let (x, y) = s
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .ok_or(format!("bad 2d point: {}", s))?;
        Ok(Point2D { x, y, added })
    }
}
fn area(p1: &Point2D, p2: &Point2D) -> u128 {
    let side = p1.x.abs_diff(p2.x) as u128;
    let height = p2.y.abs_diff(p1.y) as u128;
    (side + 1) * (height + 1)
}

#[derive(PartialEq)]
struct Pairwise {
    i1: usize,
    i2: usize,
    area: u128,
}

impl Eq for Pairwise {}

fn build_pairwise(points: &Vec<Point2D>, i1: usize, i2: usize) -> Pairwise {
    Pairwise {
        i1,
        i2,
        area: area(&points[i1], &points[i2]),
    }
}

impl PartialOrd for Pairwise {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area.partial_cmp(&other.area)
    }
}

impl Ord for Pairwise {
    fn cmp(&self, other: &Self) -> Ordering {
        self.area.cmp(&self.area)
    }
}

fn paint(points: &Vec<Point2D>, p1: &Point2D, p2: &Point2D) {
    let width = points.iter().map(|p| p.x).max().unwrap() + 300;
    let height = points.iter().map(|p| p.y).max().unwrap() + 300;
    let mut image_buf =
        image::ImageBuffer::new(width.clone() / 100 as u32, height.clone() / 100 as u32);
    for i in 0..points.len() {
        let mut x1 = points[i].x / 100;
        let mut y1 = points[i].y / 100;
        let mut x2 = points[(i + 1) % points.len()].x / 100;
        let mut y2 = points[(i + 1) % points.len()].y / 100;
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
        }
        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                let point = image_buf.get_pixel_mut(x, y);
                *point = image::Rgb([255u8, 0u8, 0u8]);
            }
        }
        let point = image_buf.get_pixel_mut(points[i].x / 100, points[i].y / 100);
        *point = image::Rgb([255u8, 255u8, 255u8]);
    }
    let smalx = p1.x.min(p2.x) / 100;
    let smaly = p1.y.min(p2.y) / 100;
    let bigx = p1.x.max(p2.x) / 100;
    let bigy = p1.y.max(p2.y) / 100;
    for x in smalx..bigx + 1 {
        for y in smaly..bigy + 1 {
            let point = image_buf.get_pixel_mut(x, y);
            *point = image::Rgb([0u8, 0u8, 255u8]);
        }
    }
    image_buf.save("202509.png").unwrap();
}
