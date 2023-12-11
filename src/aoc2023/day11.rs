use crate::common::columns::Columnser;
use crate::common::day::{Day, Question};
use itertools::Itertools;
pub struct Day11;

impl Day for Day11 {
    fn question(&self, input: &str, question: Question) {
        let res = q(input, question);
        println!("{:?}", res)
    }

    fn test_data(&self) -> String {
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .to_string()
    }
}

fn multiplier(n: usize, question: Question) -> u128 {
    match question {
        Question::First => n as u128,
        // Question::Second => (n as u128) * 1_000_000u128,
        Question::Second => (n as u128) * 999_999u128,
    }
}

fn between(r: usize, a: usize, b: usize) -> bool {
    a.min(b) < r && r < a.max(b)
}

fn q(input: &str, question: Question) -> Result<u128, String> {
    let galaxies = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(|(x, _)| (x, y))
                .collect_vec()
        })
        .flatten()
        .collect_vec();
    let width = input.lines().nth(0).unwrap().len();
    let empty_rows = input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|ch| ch != '#'))
        .map(|(row, _)| row)
        .collect_vec();
    let empty_cols = input
        .columns(width)
        .enumerate()
        .filter(|(x, col)| {
            println!("COL {}: {}", x, col);
            col.chars().all(|ch| ch != '#')
        })
        .map(|(col, _)| col)
        .collect_vec();
    println!(
        "EMPTY COLS {} ",
        empty_cols.iter().map(|c| c.to_string()).join(", ")
    );
    Ok(galaxies
        .iter()
        .combinations(2)
        .map(|gxs| {
            let ax = gxs[0].0;
            let ay = gxs[0].1;
            let bx = gxs[1].0;
            let by = gxs[1].1;
            let manhattan = ax.abs_diff(bx) + ay.abs_diff(by);
            let manhattan = manhattan as u128;
            let skipped_rows = empty_rows.iter().filter(|r| between(**r, ay, by)).count();
            let skipped_cols = empty_cols.iter().filter(|r| between(**r, ax, bx)).count();
            let skipped_rows = multiplier(skipped_rows, question);
            let skipped_cols = multiplier(skipped_cols, question);
            let total = manhattan + skipped_rows + skipped_cols;
            println!(
                "G({}, {}) <-> G({},{}) = {} + {} + {}",
                ax, ay, bx, by, manhattan, skipped_rows, skipped_cols
            );
            total
        })
        .sum())
}
