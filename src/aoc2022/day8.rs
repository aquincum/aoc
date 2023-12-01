use crate::common::day::{Day, Question};

pub struct Day8;

impl Day for Day8 {
    fn question(&self, input: &str, question: Question) {
        crate::aoc2022::day8::question(input);
    }

    fn test_data(&self) -> String {
        return "30373
25512
65332
33549
35390"
            .to_string();
    }
}

pub fn question(input: &str) {
    let tree_map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut seen = tree_map
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n_rows = tree_map.len();
    let n_cols = tree_map[0].len();

    // from the  LEFT
    for row in 0..n_rows {
        let mut current_height = -1;
        for col in 0..n_cols {
            if tree_map[row][col] > current_height {
                seen[row][col] = true;
                current_height = tree_map[row][col];
            }
        }
    }

    // from the  TOP
    for col in 0..n_cols {
        let mut current_height = -1;
        for row in 0..n_rows {
            if tree_map[row][col] > current_height {
                seen[row][col] = true;
                current_height = tree_map[row][col];
            }
        }
    }

    // from the  RIGHT
    for row in 0..n_rows {
        let mut current_height = -1;
        for col in (0..n_cols).rev() {
            if tree_map[row][col] > current_height {
                seen[row][col] = true;
                current_height = tree_map[row][col];
            }
        }
    }

    // from the  BOTTOM
    for col in 0..n_cols {
        let mut current_height = -1;
        for row in (0..n_rows).rev() {
            if tree_map[row][col] > current_height {
                seen[row][col] = true;
                current_height = tree_map[row][col];
            }
        }
    }
    println!("{:?}", seen);
    let count = seen.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc_r, seen| acc_r + *seen as i32)
    });
    println!("{}", count);

    //scenic :/

    let scenic_max = tree_map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| scenic_score(&tree_map, i, j))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    // for tree_i in 0..n_rows {
    //     for tree_j in 0..n_cols {
    //         let scenic = scenic_score(&tree_map, tree_i, tree_j);
    //     }
    // }

    println!("Scenic max: {}", scenic_max);
}

fn scenic_score(tree_map: &Vec<Vec<i32>>, pivot_row: usize, pivot_col: usize) -> u32 {
    let mut score_part = 0;
    let mut score = 1;
    let n_rows = tree_map.len();
    let n_cols = tree_map[0].len();
    let tree_height = tree_map[pivot_row][pivot_col];

    //right
    for col in (pivot_col + 1)..n_cols {
        score_part += 1;
        if tree_map[pivot_row][col] >= tree_height {
            break;
        }
    }
    println!("right: {}", score_part);
    if score_part != 0 {
        score *= score_part;
        score_part = 0;
    }
    //left
    println!("I am {} tall", tree_height);
    if pivot_col != 0 {
        for col in (0..pivot_col).rev() {
            score_part += 1;
            println!(
                "Checking if {} is taller than me...",
                tree_map[pivot_row][col]
            );
            if tree_map[pivot_row][col] >= tree_height {
                break;
            }
        }
        println!("left: {}", score_part);
        if score_part != 0 {
            score *= score_part;
            score_part = 0;
        }
    }
    //down
    for row in (pivot_row + 1)..n_rows {
        score_part += 1;
        if tree_map[row][pivot_col] >= tree_height {
            break;
        }
    }
    println!("down: {}", score_part);

    if score_part != 0 {
        score *= score_part;
        score_part = 0;
    }
    //up
    if pivot_row != 0 {
        for row in (0..pivot_row).rev() {
            score_part += 1;
            if tree_map[row][pivot_col] >= tree_height {
                break;
            }
        }
        println!("up: {}", score_part);

        if score_part != 0 {
            score *= score_part;
            score_part = 0;
        }
    }

    println!("Score for {} {}: {}", pivot_row, pivot_col, score);

    score
}
