use std::str::FromStr;
use itertools::Itertools;
use crate::common::day::{Day, Question};

pub struct Day3;

impl Day for Day3 {
    fn question(&self, input: &str, question: Question) {
        let result = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        if result.is_err() {
            println!("Error: {}", result.unwrap_err());
        } else {
            println!("{}", result.unwrap());
        }
    }

    fn test_data(&self) -> String {
        return "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    row: usize,
    from_col: usize,
    to_col: usize
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    rows: usize,
    cols: usize
}

impl Map {
    fn is_symbol(&self, row: i32, col: i32) -> bool {
        if row < 0 || row >= self.rows as i32 {
            false
        } else if col < 0 || col >= self.cols as i32{
            false
        } else {
            char_is_symbol(self.map[row as usize][col as usize])
        }
    }

    fn find_numbers(&self) -> Vec<Number> { // uuuuuuglly
        let mut nums = vec![];
        for row in 0..self.rows {
            let mut building: Option<Number> = None;
            for col in 0..self.cols {
                if self.map[row][col].is_digit(10) {
                    let val = self.map[row][col].to_digit(10).unwrap();
                    if building.is_some() {
                        let old_building = building.unwrap();
                        building = Some(Number{
                            value: old_building.value*10+val,
                            row,
                            from_col: old_building.from_col,
                            to_col: col,
                        })
                    } else {
                        building = Some(Number{
                            value: val,
                            row,
                            from_col: col,
                            to_col: col,
                        })
                    }
                } else {
                    if building.is_some() {
                        nums.push(building.unwrap());
                        building = None;
                    }
                }
            }
            if building.is_some() {
                nums.push(building.unwrap())
            }
        }
        nums
        // maybe later
        // self.map.iter().map(|row| {
        //     row.iter().fold((vec![], None), |(numbers_so_far, building), char| {
        //
        //     })
        // }).flatten().collect_vec()
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s.lines().map(|line| line.chars().collect_vec()).collect_vec();
        let rows = coords.len();
        let cols = coords[0].len();
        Ok(Map{
            map: coords,
            rows,
            cols,
        })
    }
}

impl Number {
    fn is_part(&self, map: &Map) -> bool {
        // do it ugly for now
        let from_col = self.from_col as i32;
        let row = self.row as i32;
        let to_col = self.to_col as i32;
        for col in from_col -1..to_col+2 {
            if map.is_symbol(row -1, col) || map.is_symbol(row +1, col) {
                return true;
            }
        }
        if self.value ==617 {
            println!("row={}, to_col+1={}, is_symbol={}", row, to_col+1, map.is_symbol(row,to_col+1))
        }
        if map.is_symbol(row, from_col -1) || map.is_symbol(row, to_col+1) {
            return true;
        }
        false
    }
    fn has(&self, row: i32, col: i32) -> bool {
        if row != self.row as i32 {
            return false;
        }
        col >= self.from_col as i32 && col <= self.to_col as i32
    }
}


fn char_is_symbol(ch: char) -> bool {
    !(ch.is_ascii_digit() || ch == '.')
}


fn q1(input: &str) -> Result<u32, String> {
    let map: Map = input.parse()?;
    let numbers = map.find_numbers();
    println!("Numbers: {:?}", numbers);
    let parts = numbers.iter().filter(|number| number.is_part(&map));
    println!("Parts: {:?}", numbers.iter().filter(|number| number.is_part(&map)).collect_vec());
    let sum_value = parts.map(|number| number.value).sum();

    Ok(sum_value)
}

fn q2(input: &str) -> Result<u32, String> {
    let map: Map = input.parse()?;
    let numbers = map.find_numbers();

    let mut sum: u32 = 0;
    for row in 0..(map.rows as i32) {
        for col in 0..(map.cols as i32) {
            if map.map[row as usize][col as usize] == '*' {
                let neighbors  = numbers.iter().filter(|n| {
                    n.has(row-1, col-1) || n.has(row-1, col) || n.has(row-1, col+1) ||
                    n.has(row, col-1) || n.has(row, col + 1) ||
                    n.has(row+1, col-1) || n.has(row+1, col) || n.has(row+1, col+1)
                }).collect_vec();
                if neighbors.len() == 2 {
                    sum += neighbors[0].value*neighbors[1].value
                }    let parts = numbers.iter().filter(|number| number.is_part(&map));

            }
        }
    }
    Ok(sum)
}