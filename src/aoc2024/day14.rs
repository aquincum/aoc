use crate::common::day::{Day, Question};
use image::Rgb;
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day14;

impl Day for Day14 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
            .to_string()
    }
}

struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split(" ").collect_tuple().ok_or("No 2 parts")?;
        let p = p.strip_prefix("p=").ok_or("no p= prefix")?;
        let (px, py) = p
            .split(",")
            .map(|n| n.parse().map_err(|e: ParseIntError| e.to_string()))
            .collect_tuple()
            .ok_or("No 2 parts")?;
        let v = v.strip_prefix("v=").ok_or("no v= prefix")?;
        let (vx, vy) = v
            .split(",")
            .map(|n| n.parse().map_err(|e: ParseIntError| e.to_string()))
            .collect_tuple()
            .ok_or("No 2 parts")?;
        let robot = Robot {
            x: px?,
            y: py?,
            vx: vx?,
            vy: vy?,
        };
        Ok(robot)
    }
}

impl Robot {
    fn one_move(self, width: usize, height: usize) -> Self {
        let x = self.x + self.vx.clone();
        let y = self.y + self.vy.clone();
        let x = if x < 0 {
            x + width as isize
        } else if x >= width as isize {
            x - width.clone() as isize
        } else {
            x
        };
        let y = if y < 0 {
            y + height as isize
        } else if y >= height as isize {
            y - height.clone() as isize
        } else {
            y
        };
        Robot {
            x,
            y,
            vx: self.vx,
            vy: self.vy,
        }
    }
}

fn print_robots(robots: &Vec<Robot>, width: &usize, height: &usize) {
    let mut map = (0..height.clone())
        .map(|_| (0..width.clone()).map(|_| 0usize).collect_vec())
        .collect_vec();

    for robot in robots {
        map[robot.y as usize][robot.x as usize] += 1;
    }
    for row in map {
        for ch in row {
            print!(
                "{}",
                if ch == 0 {
                    ".".to_string()
                } else {
                    ch.to_string()
                }
            );
        }
        println!();
    }
}

fn paint_robots(robots: &Vec<Robot>, width: &usize, height: &usize, filename: &str) {
    let mut image_buf = image::ImageBuffer::new(width.clone() as u32, height.clone() as u32);
    let mut dst_sum = 0f32;
    for robot in robots {
        let pix = image_buf.get_pixel_mut(robot.x as u32, robot.y as u32);
        *pix = image::Rgb([255u8, 255u8, 255u8]);
        let distance_from_center = f32::sqrt(
            (robot.x - width.clone() as isize / 2).pow(2) as f32
                + (robot.y - height.clone() as isize / 2).pow(2) as f32,
        );
        dst_sum += distance_from_center;
    }
    let mean_dist = dst_sum / robots.len() as f32;
    if mean_dist < 28f32 {
        println!("{} -- {}", filename, mean_dist);
        image_buf.save(filename).unwrap();
    }
}

fn q1(input: &str) -> Result<u128, String> {
    let robots: Result<Vec<Robot>, String> = input.lines().map(|l| l.parse()).collect();
    let robots = robots?;
    let (width, height) = if robots[0].x == 0 {
        // test data lol
        (11usize, 7usize)
    } else {
        (101usize, 103usize)
    };
    print_robots(&robots, &width, &height);
    let robots = (0..100).fold(robots, |robots, i| {
        robots
            .into_iter()
            .map(|r| r.one_move(width, height))
            .collect()
    });
    println!("===========");
    print_robots(&robots, &width, &height);
    let quadrants =
        robots.iter().fold(
            (0u128, 0u128, 0u128, 0u128),
            |(ul, ur, bl, br), robot| match (robot.x.clone() as usize, robot.y.clone() as usize) {
                (x, y) if x < (width.clone() - 1) / 2 && y < (height.clone() - 1) / 2 => {
                    (ul + 1, ur, bl, br)
                }
                (x, y) if x < (width.clone() - 1) / 2 && y > (height.clone() - 1) / 2 => {
                    (ul, ur, bl + 1, br)
                }
                (x, y) if x > (width.clone() - 1) / 2 && y < (height.clone() - 1) / 2 => {
                    (ul, ur + 1, bl, br)
                }
                (x, y) if x > (width.clone() - 1) / 2 && y > (height.clone() - 1) / 2 => {
                    (ul, ur, bl, br + 1)
                }
                _ => (ul, ur, bl, br),
            },
        );
    Ok(quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3 as u128)
}

fn q2(input: &str) -> Result<u128, String> {
    let mut robots: Result<Vec<Robot>, String> = input.lines().map(|l| l.parse()).collect();
    let mut robots = robots?;
    let (width, height) = (101usize, 103usize);
    for i in 0..10000 {
        paint_robots(
            &robots,
            &width,
            &height,
            &format!("test{:0>4}.png", i.to_string()),
        );
        robots = robots
            .into_iter()
            .map(|r| r.one_move(width.clone(), height.clone()))
            .collect_vec();
    }
    Ok((1))
}
