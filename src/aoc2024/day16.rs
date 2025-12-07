use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::ops::Index;
use std::str::FromStr;

pub struct Day16;

impl Day for Day16 {
    fn question(&self, input: &str, question: Question) {
        let mut map: Map = input.parse().unwrap();
        let start = map.find_unique_pixel(Pixel::Start);
        // let (cost, route) = map
        //     .dist_from_end(start, Direction::Right, HashSet::new())
        //     .unwrap();
        // println!("{:?}", cost);
        // map.print_with_route(route);
        let bests = map.dyna_solve();
        let start = map.find_unique_pixel(Pixel::Start);
        let start_best = bests[start.0][start.1];
        if start_best.is_none() {
            println!("RUH ROH NO SOLN!?");
        } else {
            let start_best = start_best.unwrap();
            let val = match start_best.direction {
                None => 0,
                Some(Direction::Up) => start_best.cost + 1000,
                Some(Direction::Down) => start_best.cost + 1000,
                Some(Direction::Left) => start_best.cost,
                Some(Direction::Right) => start_best.cost + 2000,
            };
            println!("{}", val);
            println!("{:?}", start_best.direction);
        }

        for y in 0..map.height() {
            for x in 0..map.width() {
                if let Some(best) = bests[x][y] {
                    print!("{: >6}", best.cost);
                } else {
                    print!(" #### ");
                }
            }
            println!();
        }

        let list_of_besties = map.find_best_paths(bests);

        for y in 0..map.height() {
            for x in 0..map.width() {
                if list_of_besties.contains(&(x, y)) {
                    print!("O");
                } else {
                    print!("{}", map[(x, y)]);
                }
            }
            println!();
        }

        println!("{}", list_of_besties.len());
    }

    fn test_data(&self) -> String {
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            .to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pixel {
    Wall,
    Corridor,
    End,
    Start,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::Wall => '#',
                Pixel::Corridor => '.',
                Pixel::End => 'E',
                Pixel::Start => 'S',
            }
        )
    }
}

impl From<char> for Pixel {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Pixel::Wall,
            '.' => Pixel::Corridor,
            'E' => Pixel::End,
            'S' => Pixel::Start,
            _ => panic!("illegal char {}", ch),
        }
    }
}

type Coord = (usize, usize);
struct Map(Vec<Vec<Pixel>>);

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map(s
            .lines()
            .map(|row| row.chars().map(|ch| Pixel::from(ch)).collect_vec())
            .collect_vec()))
    }
}

impl Map {
    fn find_unique_pixel(&self, find_what: Pixel) -> Coord {
        for (coord, pixel) in self.iter() {
            if pixel == find_what {
                return coord;
            }
        }
        panic!("nonsense")
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }
    fn iter(&self) -> MapAllPixelsIterator {
        MapAllPixelsIterator {
            map: self,
            current: (0, 0),
        }
    }
    // what a lovely functional pure happy function that works on the test but obviously not on
    // the main question because :(
    fn dist_from_end(
        &self,
        coord: Coord,
        facing: Direction,
        mut visited: HashSet<Coord>,
    ) -> Option<(u128, Vec<Coord>)> {
        if visited.contains(&coord) {
            None
        } else if self[coord] == Pixel::Wall {
            None
        } else if self[coord] == Pixel::End {
            Some((0, vec![coord]))
        } else {
            let (x, y) = coord;
            visited.insert(coord.clone());
            let plans = match facing {
                Direction::Up => [
                    Plan {
                        coord: (x, y - 1),
                        facing: Direction::Up,
                        cost: 1,
                    },
                    Plan {
                        coord: (x - 1, y),
                        facing: Direction::Left,
                        cost: 1001,
                    },
                    Plan {
                        coord: (x + 1, y),
                        facing: Direction::Right,
                        cost: 1001,
                    },
                ],
                Direction::Down => [
                    Plan {
                        coord: (x, y + 1),
                        facing: Direction::Down,
                        cost: 1,
                    },
                    Plan {
                        coord: (x - 1, y),
                        facing: Direction::Left,
                        cost: 1001,
                    },
                    Plan {
                        coord: (x + 1, y),
                        facing: Direction::Right,
                        cost: 1001,
                    },
                ],

                Direction::Left => [
                    Plan {
                        coord: (x, y - 1),
                        facing: Direction::Up,
                        cost: 1001,
                    },
                    Plan {
                        coord: (x - 1, y),
                        facing: Direction::Left,
                        cost: 1,
                    },
                    Plan {
                        coord: (x, y + 1),
                        facing: Direction::Down,
                        cost: 1001,
                    },
                ],

                Direction::Right => [
                    Plan {
                        coord: (x, y - 1),
                        facing: Direction::Up,
                        cost: 1001,
                    },
                    Plan {
                        coord: (x + 1, y),
                        facing: Direction::Right,
                        cost: 1,
                    },
                    Plan {
                        coord: (x, y + 1),
                        facing: Direction::Down,
                        cost: 1001,
                    },
                ],
            };
            plans
                .iter()
                .map(|x| {
                    (
                        self.dist_from_end(x.coord, x.facing, visited.clone()),
                        x.cost,
                    )
                })
                .filter(|(result, _)| result.is_some())
                .map(|(result, cost)| {
                    let (subcost, route) = result.unwrap();
                    (subcost + cost as u128, vec![route, vec![coord]].concat())
                })
                .min_by_key(|result| result.0)
        }
    }
    fn dyna_solve(&self) -> Vec<Vec<Option<Best>>> {
        let mut bests_map = vec![vec![None; self.height()]; self.width()];
        let (endx, endy) = self.find_unique_pixel(Pixel::End);
        bests_map[endx][endy] = Some(Best {
            cost: 0,
            direction: None,
            coord: (endx, endy),
        });
        let mut do_queue = VecDeque::from([bests_map[endx][endy].unwrap()]);
        while let Some(curr) = do_queue.pop_front() {
            let left = DirectionPlanning {
                coord: (curr.coord.0 - 1, curr.coord.1),
                best_sofar: bests_map[curr.coord.0 - 1][curr.coord.1],
                ours: curr.cost
                    + match curr.direction {
                        None => 1,
                        Some(Direction::Up) => 1001,
                        Some(Direction::Down) => 1001,
                        Some(Direction::Left) => 1,
                        Some(Direction::Right) => 2001,
                    },
                direction: Direction::Left,
            };
            let right = DirectionPlanning {
                coord: (curr.coord.0 + 1, curr.coord.1),
                best_sofar: bests_map[curr.coord.0 + 1][curr.coord.1],
                ours: curr.cost
                    + match curr.direction {
                        None => 1,
                        Some(Direction::Up) => 1001,
                        Some(Direction::Down) => 1001,
                        Some(Direction::Left) => 2001,
                        Some(Direction::Right) => 1,
                    },
                direction: Direction::Right,
            };
            let up = DirectionPlanning {
                coord: (curr.coord.0, curr.coord.1 - 1),
                best_sofar: bests_map[curr.coord.0][curr.coord.1 - 1],
                ours: curr.cost
                    + match curr.direction {
                        None => 1,
                        Some(Direction::Up) => 1,
                        Some(Direction::Down) => 2001,
                        Some(Direction::Left) => 1001,
                        Some(Direction::Right) => 1001,
                    },
                direction: Direction::Up,
            };
            let down = DirectionPlanning {
                coord: (curr.coord.0, curr.coord.1 + 1),
                best_sofar: bests_map[curr.coord.0][curr.coord.1 + 1],
                ours: curr.cost
                    + match curr.direction {
                        None => 1,
                        Some(Direction::Up) => 2001,
                        Some(Direction::Down) => 1,
                        Some(Direction::Left) => 1001,
                        Some(Direction::Right) => 1001,
                    },
                direction: Direction::Down,
            };
            for attempt in [left, right, up, down] {
                if self[attempt.coord] == Pixel::Wall {
                    continue;
                }
                if attempt.best_sofar.is_none() || attempt.best_sofar.unwrap().cost > attempt.ours {
                    bests_map[attempt.coord.0][attempt.coord.1] = Some(Best {
                        cost: attempt.ours,
                        direction: Some(attempt.direction),
                        coord: attempt.coord,
                    });
                    do_queue.push_back(bests_map[attempt.coord.0][attempt.coord.1].unwrap());
                } else if !attempt.best_sofar.is_none()
                    && attempt.best_sofar.unwrap().cost + 1000 > attempt.ours
                {
                    // i think this is a possibility
                    // apparetnyl not
                    do_queue.push_back(Best {
                        cost: attempt.ours,
                        direction: Some(attempt.direction),
                        coord: attempt.coord,
                    });
                }
            }
        }
        bests_map
    }
    fn find_best_paths(&self, bests_map: Vec<Vec<Option<Best>>>) -> HashSet<Coord> {
        let starter = self.find_unique_pixel(Pixel::Start);
        let mut do_queue: VecDeque<(Coord, Direction)> = VecDeque::from([(
            starter,
            bests_map[starter.0][starter.1].unwrap().direction.unwrap(),
        )]);
        let mut results = HashSet::new();
        while let Some((curr, prev_dir)) = do_queue.pop_front() {
            if results.contains(&curr) {
                continue;
            }
            if let Some(best) = bests_map[curr.0][curr.1] {
                results.insert(best.coord);
                if prev_dir == Direction::Left {
                    // balrol jottem
                    if bests_map[curr.0 + 1][curr.1]
                        .map(|v| v.cost + 1 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0 + 1, curr.1), Direction::Left))
                    }
                    if bests_map[curr.0 + 1][curr.1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front((
                            (curr.0 + 1, curr.1),
                            bests_map[curr.0 + 1][curr.1].unwrap().direction.unwrap(),
                        ))
                    }

                    if bests_map[curr.0][curr.1 - 1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0, curr.1 - 1), Direction::Down))
                    }
                    if bests_map[curr.0][curr.1 + 1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0, curr.1 + 1), Direction::Down))
                    }
                }
                if prev_dir == Direction::Right {
                    // jobbrol jottem
                    if bests_map[curr.0 - 1][curr.1]
                        .map(|v| v.cost + 1 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0 - 1, curr.1), Direction::Right))
                    }
                    if bests_map[curr.0 - 1][curr.1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front((
                            (curr.0 - 1, curr.1),
                            bests_map[curr.0 - 1][curr.1].unwrap().direction.unwrap(),
                        ))
                    }

                    if bests_map[curr.0][curr.1 - 1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0, curr.1 - 1), Direction::Down))
                    }
                    if bests_map[curr.0][curr.1 + 1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0, curr.1 + 1), Direction::Down))
                    }
                }
                if prev_dir == Direction::Up {
                    // fentrol jottem
                    if bests_map[curr.0][curr.1 + 1]
                        .map(|v| v.cost + 1 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0, curr.1 + 1), Direction::Up))
                    }
                    if bests_map[curr.0][curr.1 + 1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front((
                            (curr.0, curr.1 + 1),
                            bests_map[curr.0][curr.1 + 1].unwrap().direction.unwrap(),
                        ))
                    }

                    if bests_map[curr.0 - 1][curr.1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0 - 1, curr.1), Direction::Right))
                    }
                    if bests_map[curr.0 + 1][curr.1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0 + 1, curr.1), Direction::Left))
                    }
                }
                if prev_dir == Direction::Down {
                    // lentrol jottem
                    if bests_map[curr.0][curr.1 - 1]
                        .map(|v| v.cost + 1 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0, curr.1 - 1), Direction::Down))
                    }
                    if bests_map[curr.0][curr.1 - 1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front((
                            (curr.0, curr.1 - 1),
                            bests_map[curr.0][curr.1 - 1].unwrap().direction.unwrap(),
                        ))
                    }
                    if bests_map[curr.0 - 1][curr.1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0 - 1, curr.1), Direction::Right))
                    }
                    if bests_map[curr.0 + 1][curr.1]
                        .map(|v| v.cost + 1001 == best.cost)
                        .unwrap_or(false)
                    {
                        do_queue.push_front(((curr.0 + 1, curr.1), Direction::Left))
                    }
                }

                // if best.direction == Some(Direction::Left) {
                //     do_queue.push_front((curr.0 + 1, curr.1));
                //     if bests_map[curr.0][curr.1 - 1]
                //         .map(|v| v.cost == best.cost + 999)
                //         .unwrap_or(false)
                //     {
                //         do_queue.push_front((curr.0, curr.1 - 1));
                //     }
                //     if bests_map[curr.0][curr.1 + 1]
                //         .map(|v| v.cost == best.cost + 999)
                //         .unwrap_or(false)
                //     {
                //         do_queue.push_front((curr.0, curr.1 + 1));
                //     }
                // }
                // if best.direction == Some(Direction::Right) {
                //     do_queue.push_front((curr.0 - 1, curr.1));
                //     if bests_map[curr.0][curr.1 - 1]
                //         .map(|v| v.cost == best.cost + 999)
                //         .unwrap_or(false)
                //     {
                //         do_queue.push_front((curr.0, curr.1 - 1));
                //     }
                //     if bests_map[curr.0][curr.1 + 1]
                //         .map(|v| v.cost == best.cost + 999)
                //         .unwrap_or(false)
                //     {
                //         do_queue.push_front((curr.0, curr.1 + 1));
                //     }
                // }
                // if best.direction == Some(Direction::Up) {
                //     do_queue.push_front((curr.0, curr.1 + 1));
                //     if bests_map[curr.0 - 1][curr.1]
                //         .map(|v| v.cost == best.cost + 999)
                //         .unwrap_or(false)
                //     {
                //         do_queue.push_front((curr.0 - 1, curr.1));
                //     }
                //     if bests_map[curr.0 + 1][curr.1]
                //         .map(|v| v.cost == best.cost + 999)
                //         .unwrap_or(false)
                //     {
                //         do_queue.push_front((curr.0 + 1, curr.1));
                //     }
                // }
                // if best.direction == Some(Direction::Down) {
                //     do_queue.push_front((curr.0, curr.1 - 1));
                //     if bests_map[curr.0 - 1][curr.1]
                //         .map(|v| v.cost == best.cost + 999)
                //         .unwrap_or(false)
                //     {
                //         do_queue.push_front((curr.0 - 1, curr.1));
                //     }
                //     if bests_map[curr.0 + 1][curr.1]
                //         .map(|v| v.cost == best.cost + 999)
                //         .unwrap_or(false)
                //     {
                //         do_queue.push_front((curr.0 + 1, curr.1));
                //     }
                // }

                // if bests_map[curr.0 - 1][curr.1]
                //     .map(|v| v.cost < best.cost)
                //     .unwrap_or(false)
                // {
                //     do_queue.push_front((curr.0 - 1, curr.1));
                // }
                // if curr.0 > 1
                //     && bests_map[curr.0 - 2][curr.1]
                //         .map(|v| v.cost + 2 == best.cost)
                //         .unwrap_or(false)
                // {
                //     do_queue.push_front((curr.0 - 2, curr.1));
                // }
                // if bests_map[curr.0 + 1][curr.1]
                //     .map(|v| v.cost < best.cost)
                //     .unwrap_or(false)
                // {
                //     do_queue.push_front((curr.0 + 1, curr.1));
                // }
                // if curr.0 < self.width() - 2
                //     && bests_map[curr.0 + 2][curr.1]
                //         .map(|v| v.cost + 2 == best.cost)
                //         .unwrap_or(false)
                // {
                //     do_queue.push_front((curr.0 + 2, curr.1));
                // }
                // if bests_map[curr.0][curr.1 - 1]
                //     .map(|v| v.cost < best.cost)
                //     .unwrap_or(false)
                // {
                //     do_queue.push_front((curr.0, curr.1 - 1));
                // }
                // if curr.1 > 1
                //     && bests_map[curr.0][curr.1 - 2]
                //         .map(|v| v.cost + 2 == best.cost)
                //         .unwrap_or(false)
                // {
                //     do_queue.push_front((curr.0, curr.1 - 2));
                // }
                // if bests_map[curr.0][curr.1 + 1]
                //     .map(|v| v.cost < best.cost)
                //     .unwrap_or(false)
                // {
                //     do_queue.push_front((curr.0, curr.1 + 1));
                // }
                // if curr.1 < self.height() - 2
                //     && bests_map[curr.0][curr.1 + 2]
                //         .map(|v| v.cost + 2 == best.cost)
                //         .unwrap_or(false)
                // {
                //     do_queue.push_front((curr.0, curr.1 + 2));
                // }
            }
        }
        results
    }
    fn print_with_route(&self, route: Vec<Coord>) {
        let route: HashSet<Coord> = route.into_iter().collect();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if route.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!("{}", self[(x, y)]);
                }
            }
            println!();
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Best {
    cost: u128,
    coord: Coord,
    direction: Option<Direction>,
}

//?
#[derive(Copy, Clone)]
struct DirectionPlanning {
    coord: Coord,
    best_sofar: Option<Best>,
    ours: u128,
    direction: Direction,
}

#[derive(Copy, Clone)]
struct Plan {
    coord: Coord,
    facing: Direction,
    cost: usize,
}

// little util attempts

impl Index<Coord> for Map {
    type Output = Pixel;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

struct MapAllPixelsIterator<'a> {
    map: &'a Map,
    current: Coord,
}

impl Iterator for MapAllPixelsIterator<'_> {
    type Item = (Coord, Pixel);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.current.clone();
        if y == self.map.height() {
            None
        } else {
            if x + 1 == self.map.width() {
                self.current = (0, y + 1)
            } else {
                self.current = (x + 1, y)
            }

            Some(((x.clone(), y.clone()), self.map[(x, y)]))
        }
    }
}
