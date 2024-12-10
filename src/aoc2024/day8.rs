use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day8;

impl Day for Day8 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string()
    }
}

struct Antenna {
    x: usize,
    y: usize,
    name: char,
}

fn collect_antennas(input: &str) -> Vec<Antenna> {
    input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars().enumerate().map(move |(x, ch)| match ch {
                '.' => None,
                name => Some(Antenna { x, y, name }),
            })
        })
        .flatten()
        .filter_map(|o| o)
        .collect_vec()
}

fn fits(x: isize, y: isize, width: usize, height: usize) -> bool {
    !(x < 0 || x >= width as isize || y < 0 || y >= height as isize)
}

fn get_antipodes(
    group: Vec<&Antenna>,
    question: Question,
    width: usize,
    height: usize,
) -> Vec<(isize, isize)> {
    group
        .iter()
        .tuple_combinations()
        .map(|(a, b): (&&Antenna, &&Antenna)| {
            if question == Question::First {
                let diffx = a.x as isize - b.x as isize;
                let diffy = a.y as isize - b.y as isize;
                let newax = a.x as isize + diffx;
                let neway = a.y as isize + diffy;
                let newbx = b.x as isize - diffx;
                let newby = b.y as isize - diffy;
                vec![(newax, neway), (newbx, newby)]
            } else {
                let mut results = vec![];
                let diffx = a.x as isize - b.x as isize;
                let diffy = a.y as isize - b.y as isize;
                let mut curx = a.x as isize;
                let mut cury = a.y as isize;
                while curx >= 0 && curx < width as isize && cury >= 0 && cury < height as isize {
                    results.push((curx, cury));
                    curx += diffx;
                    cury += diffy;
                }

                let mut curx = b.x as isize;
                let mut cury = b.y as isize;
                while curx >= 0 && curx < width as isize && cury >= 0 && cury < height as isize {
                    results.push((curx, cury));
                    curx -= diffx;
                    cury -= diffy;
                }

                results
            }
        })
        .flatten()
        .unique()
        .collect()
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let antennas = collect_antennas(input);
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    println!("{}", antennas.len());

    // antennas.group_by(|x, y| x.name == y.name);
    let antipodes: Vec<(isize, isize)> = antennas
        .iter()
        .sorted_by_key(|a| a.name)
        .group_by(|x| x.name)
        .into_iter()
        .map(|(ch, group)| get_antipodes(group.collect(), question, width, height))
        .into_iter()
        .flatten()
        .unique()
        .filter(|(x, y)| fits(*x, *y, width, height))
        .collect_vec();

    // print_situation(antennas, height, width);
    Ok(antipodes.len() as u128)
}

fn print_situation(antennas: Vec<Antenna>, height: usize, width: usize, question: Question) {
    for ch in antennas.iter().map(|a| a.name).unique().sorted() {
        let ants = antennas.iter().filter(|a| a.name == ch).collect_vec();
        let aps: HashSet<(isize, isize)> = get_antipodes(ants, question, width, height)
            .into_iter()
            .unique()
            .filter(|(x, y)| fits(*x, *y, width, height))
            .collect();
        let ants: HashSet<(isize, isize)> = antennas
            .iter()
            .filter(|a| a.name == ch)
            .map(|a| (a.x as isize, a.y as isize))
            .collect();
        for y in 0..height {
            for x in 0..width {
                if aps.contains(&(x as isize, y as isize)) {
                    print!("#");
                } else if ants.contains(&(x as isize, y as isize)) {
                    print!("{}", ch);
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!()
    }
}

//434 too hi
