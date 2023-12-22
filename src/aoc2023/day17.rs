use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::str::FromStr;

pub struct Day17;

impl Day for Day17 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
            .to_string()
    }
}

type Optimum = Option<usize>;

#[derive(Debug)]
struct Optima((Optimum, Optimum, Optimum));

impl Optima {
    fn best(&self) -> Optimum {
        match self.0 {
            (None, None, None) => None,
            (Some(x), None, None) => Some(x),
            (Some(x), Some(y), None) => Some(x.min(y)),
            (Some(x), Some(y), Some(z)) => Some(x.min(y.min(z))),
            (None, Some(y), None) => Some(y),
            (None, None, Some(z)) => Some(z),
            (None, Some(y), Some(z)) => Some(y.min(z)),
            (Some(x), None, Some(z)) => Some(x.min(z)),
            // _ => panic!(format!("pay attention {:?}", self.0)),
        }
    }
    fn add(self, heatloss: &usize) -> Self {
        Optima((
            self.0 .0.map(|x| x + heatloss),
            self.0 .1.map(|x| x + heatloss),
            self.0 .2.map(|x| x + heatloss),
        ))
    }
}

struct Block {
    heatloss: usize,
    from_left: Optima,
    from_top: Optima,
    from_right: Optima,
    from_bottom: Optima,
    where_from: Option<ComingFrom>,
}

struct Map(Vec<Vec<Block>>);

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|row| {
                row.chars()
                    .map(|ch| Block {
                        heatloss: ch.to_digit(10).unwrap() as usize,
                        from_left: Optima((None, None, None)),
                        from_top: Optima((None, None, None)),
                        from_right: Optima((None, None, None)),
                        from_bottom: Optima((None, None, None)),
                        where_from: None,
                    })
                    .collect_vec()
            })
            .collect_vec();
        Ok(Map(map))
    }
}

#[derive(Copy, Clone)]
enum ComingFrom {
    Left,
    Right,
    Up,
    Down,
}

impl Map {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
    fn update_all(&mut self) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.update_cell(i, j);
            }
        }
    }
    fn update_cell(&mut self, row: usize, col: usize) {
        if row > 0 {
            self.0[row][col].from_top = self
                .updated_value_from(row - 1, col, ComingFrom::Up)
                .add(&self.0[row][col].heatloss);
        }
        if row < self.height() - 1 {
            self.0[row][col].from_bottom = self
                .updated_value_from(row + 1, col, ComingFrom::Down)
                .add(&self.0[row][col].heatloss);
        }
        if col > 0 {
            self.0[row][col].from_left = self
                .updated_value_from(row, col - 1, ComingFrom::Left)
                .add(&self.0[row][col].heatloss);
        }
        if col < self.width() - 1 {
            self.0[row][col].from_right = self
                .updated_value_from(row, col + 1, ComingFrom::Right)
                .add(&self.0[row][col].heatloss);
        }
    }

    fn updated_value_from(&self, row: usize, col: usize, direction: ComingFrom) -> Optima {
        let cell = &self.0[row][col];
        let best = self.best_for_except(row, col, direction);
        let optima = match direction {
            ComingFrom::Left => &cell.from_left,
            ComingFrom::Right => &cell.from_right,
            ComingFrom::Up => &cell.from_top,
            ComingFrom::Down => &cell.from_bottom,
        };
        Optima((best, optima.0 .0, optima.0 .1))
    }

    fn best_for(&self, row: usize, col: usize) -> Option<(ComingFrom, usize)> {
        let cell = &self.0[row][col];
        [
            (ComingFrom::Down, cell.from_bottom.best()),
            (ComingFrom::Right, cell.from_right.best()),
            (ComingFrom::Up, cell.from_top.best()),
            (ComingFrom::Left, cell.from_left.best()),
        ]
        .into_iter()
        .filter_map(|(n, o)| match o {
            None => None,
            Some(o) => Some((n, o)),
        })
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(n, o)| (*n, *o))
    }
    fn best_for_except(&self, row: usize, col: usize, direction: ComingFrom) -> Optimum {
        let cell = &self.0[row][col];
        let b = cell.from_bottom.best();
        let r = cell.from_right.best();
        let t = cell.from_top.best();
        let l = cell.from_left.best();
        let opts = match direction {
            ComingFrom::Left => [b, t, r],
            ComingFrom::Right => [b, t, l],
            ComingFrom::Up => [b, r, l],
            ComingFrom::Down => [t, r, l],
        };
        opts.iter().filter_map(|o| *o).min()
    }
    fn print_best(&self) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                print!("{} ", self.best_for(i, j).map(|x| x.1).unwrap_or(0))
            }
            println!();
        }
    }
    fn print_route(&self) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                print!(
                    "{}",
                    self.best_for(i, j)
                        .map(|x| x.0)
                        .map(|cf| match cf {
                            ComingFrom::Left => '>',
                            ComingFrom::Right => '<',
                            ComingFrom::Up => 'v',
                            ComingFrom::Down => '^',
                        })
                        .unwrap_or('.')
                )
            }
            println!();
        }
    }
}

fn print_route_map(map: &Map) {
    let mut rtemap = map
        .0
        .iter()
        .map(|r| r.iter().map(|_| '.').collect_vec())
        .collect_vec();
    let mut x = (map.width() - 1) as isize;
    let mut y = (map.height() - 1) as isize;
    while x != 0 && y != 0 {
        let (from, best) = map.best_for(x as usize, y as usize).unwrap();
        match from {
            ComingFrom::Left => {
                rtemap[x as usize][y as usize] = '>';
                let f = &map.0[x as usize][y as usize].from_left;
                y -= if f.0 .2.map(|n| best == n).unwrap_or(false) {
                    3
                } else if f.0 .1.map(|n| best == n).unwrap_or(false) {
                    2
                } else {
                    1
                }
            }
            ComingFrom::Right => {
                rtemap[x as usize][y as usize] = '<';
                let f = &map.0[x as usize][y as usize].from_right;
                y += if f.0 .2.map(|n| best == n).unwrap_or(false) {
                    3
                } else if f.0 .1.map(|n| best == n).unwrap_or(false) {
                    2
                } else {
                    1
                }
            }
            ComingFrom::Up => {
                rtemap[x as usize][y as usize] = 'v';
                let f = &map.0[x as usize][y as usize].from_top;
                x -= if f.0 .2.map(|n| best == n).unwrap_or(false) {
                    3
                } else if f.0 .1.map(|n| best == n).unwrap_or(false) {
                    2
                } else {
                    1
                }
            }
            ComingFrom::Down => {
                rtemap[x as usize][y as usize] = '^';
                let f = &map.0[x as usize][y as usize].from_bottom;

                x += if f.0 .2.map(|n| best == n).unwrap_or(false) {
                    3
                } else if f.0 .1.map(|n| best == n).unwrap_or(false) {
                    2
                } else {
                    1
                }
            }
        }
        if x < 0 || y < 0 {
            println!("NAGY BAJBA VAGYOK {} {}", x, y);
            break;
        }
    }
    // if map.0[x][y]
    //     .from_left
    //     .best()
    //     .map(|b| b == best)
    //     .unwrap_or(false)
    // {
    //     print!(">");
    //     x -= 1;
    // } else if map.0[x][y]
    //     .from_top
    //     .best()
    //     .map(|b| b == best)
    //     .unwrap_or(false)
    // {
    //     print!("v");
    //     y -= 1;
    // } else if map.0[x][y]
    //     .from_bottom
    //     .best()
    //     .map(|b| b == best)
    //     .unwrap_or(false)
    // {
    //     print!("^");
    //     y += 1;
    // } else if map.0[x][y]
    //     .from_right
    //     .best()
    //     .map(|b| b == best)
    //     .unwrap_or(false)
    // {
    //     print!("<");
    //     x += 1;
    // } else {
    //     println!("Situation! [{}, {}] best {} has:", x, y, best);
    //     println!("From the left: {:?}", map.0[x][y].from_left);
    //     println!("From the right: {:?}", map.0[x][y].from_right);
    //     println!("From the top: {:?}", map.0[x][y].from_top);
    //     println!("From the bottom: {:?}", map.0[x][y].from_bottom);
    //     println!("My heat is {}", map.0[x][y].heatloss);
    //     return;
    // }
    // }

    for row in rtemap {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

fn q1(input: &str) -> Result<usize, String> {
    let mut map = input.parse::<Map>()?;
    // map.0[0][1].from_left = Optima((Some(0), None, None));
    // map.0[1][0].from_top = Optima((Some(0), None, None));
    map.0[0][0].from_left = Optima((Some(0), Some(0), Some(0)));
    map.0[0][0].from_top = Optima((Some(0), Some(0), Some(0)));
    let heatloss = map.0[map.height() - 1][map.width() - 1].heatloss;
    map.update_all();
    map.print_best();
    map.update_all();
    map.print_best();
    map.update_all();
    map.print_best();
    map.update_all();
    map.print_best();
    let from_up = map.updated_value_from(map.height() - 1, map.width() - 1, ComingFrom::Up);
    let from_left = map.updated_value_from(map.height() - 1, map.width() - 1, ComingFrom::Left);
    map.print_route();
    println!("{:?} {:?}", from_up, from_left);
    let best = map.best_for(map.height() - 1, map.width() - 1);
    print_route_map(&map);
    best.map(|x| x.1 + heatloss)
        .ok_or("no best found".to_string())
}
fn q2(input: &str) -> Result<usize, String> {
    todo!()
}
