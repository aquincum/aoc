use crate::common::day::{Day, Question};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
use std::str::FromStr;

pub struct Day22;

impl Day for Day22 {
    fn question(&self, input: &str, question: Question) {
        let res = match question {
            Question::First => q1(input),
            Question::Second => q2(input),
        };
        println!("{:?}", res);
    }

    fn test_data(&self) -> String {
        "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
            .to_string()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z == rhs.z;
    }
}

impl FromStr for Coord {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .ok_or(())?;
        Ok(Coord { x, y, z })
    }
}

const UNIT_Z: Coord = Coord { x: 0, y: 0, z: 1 };

type BlockId = usize;

#[derive(Clone)]
struct Block {
    id: BlockId,
    coords: Vec<Coord>,
}

impl Block {
    fn new(block_id: BlockId, input: &str) -> Block {
        let (begin, end) = input
            .split("~")
            .map(|s| s.parse::<Coord>().unwrap())
            .collect_tuple()
            .unwrap();
        if begin == end {
            return Block {
                id: block_id,
                coords: vec![begin],
            };
        }
        let inbetweens = if begin.x != end.x {
            (begin.x.min(end.x)..begin.x.max(end.x))
                .map(|x| Coord {
                    x,
                    y: begin.y,
                    z: begin.z,
                })
                .collect_vec()
        } else if begin.y != end.y {
            (begin.y.min(end.y)..begin.y.max(end.y))
                .map(|y| Coord {
                    x: begin.x,
                    y,
                    z: begin.z,
                })
                .collect_vec()
        } else {
            (begin.z.min(end.z)..begin.z.max(end.z))
                .map(|z| Coord {
                    x: begin.x,
                    y: begin.y,
                    z,
                })
                .collect_vec()
        };
        let coords = vec![vec![begin], inbetweens, vec![end]]
            .into_iter()
            .flatten()
            .collect_vec();
        Block {
            id: block_id,
            coords,
        }
    }
    fn is_vertical(&self) -> bool {
        self.coords.first().unwrap().z != self.coords.last().unwrap().z
    }
}

struct State {
    blocks: HashMap<BlockId, Block>,
    points: Vec<Option<BlockId>>,
    x_dim: usize,
    y_dim: usize,
    z_dim: usize,
}

impl State {
    fn new(blocks: HashMap<BlockId, Block>) -> Self {
        let (x_dim, y_dim, z_dim) = blocks
            .values()
            .map(|block| {
                let first = block.coords.first().unwrap();
                let last = block.coords.last().unwrap();
                (
                    first.x.max(last.x),
                    first.y.max(last.y),
                    first.z.max(last.z),
                )
            })
            .fold((0, 0, 0), |acc, curr| {
                (acc.0.max(curr.0), acc.1.max(curr.1), acc.2.max(curr.2))
            });
        let (x_dim, y_dim, z_dim) = (x_dim + 1, y_dim + 1, z_dim + 1);
        let mut points = vec![None; x_dim * y_dim * z_dim];
        for (block_id, block) in &blocks {
            for coord in &block.coords {
                points[coord.x + coord.y * x_dim + coord.z * y_dim * x_dim] = Some(*block_id);
            }
        }
        State {
            blocks,
            points,
            x_dim,
            y_dim,
            z_dim,
        }
    }

    fn get_point(&self, coord: Coord) -> &Option<BlockId> {
        &self.points[coord.x + coord.y * self.x_dim + coord.z * self.y_dim * self.x_dim]
        // z is the biggest # usually
    }

    fn set_point(&mut self, coord: Coord, value: Option<BlockId>) {
        self.points[coord.x + coord.y * self.x_dim + coord.z * self.y_dim * self.x_dim] = value
    }

    fn print_from_x(&self) {
        for z in (1..self.z_dim).rev() {
            'xloop: for x in 0..self.x_dim {
                for y in 0..self.y_dim {
                    if let Some(block_id) = self.get_point(Coord { x, y, z }) {
                        print!("{}", block_id);
                        continue 'xloop;
                    }
                }
                print!(".");
            }
            println!(" {}", z);
        }
    }

    fn is_droppable(&self, block: &Block) -> bool {
        if block.is_vertical() {
            // vertical block
            let below = *block.coords.first().unwrap() - UNIT_Z;
            below.z != 0 && self[below].is_none()
        } else {
            block.coords.iter().all(|&coord| {
                let below = coord - UNIT_Z;
                below.z != 0 && self[below].is_none()
            })
        }
    }

    fn drop_block(&mut self, block: &mut Block) {
        for coord in block.coords.iter() {
            self[*coord] = None;
        }
        block
            .coords
            .iter_mut()
            .for_each(|mut coord| *coord -= UNIT_Z);
        for coord in block.coords.iter() {
            self[*coord] = Some(block.id);
        }
    }

    fn supports(&self, block_id: BlockId) -> HashSet<BlockId> {
        let block = &self.blocks.get(&block_id).unwrap();
        let block_coords = &block.coords;
        if block.is_vertical() {
            let on_top = block_coords.iter().max_by(|a, b| a.z.cmp(&b.z)).unwrap();
            let on_top = *on_top + UNIT_Z;
            if let Some(bid) = self[on_top] {
                HashSet::from([bid])
            } else {
                HashSet::new()
            }
        } else {
            block_coords
                .iter()
                .map(|c| *c + UNIT_Z)
                .filter_map(|coord| self[coord])
                .collect()
        }
    }
    fn supported_by(&self, block_id: BlockId) -> HashSet<BlockId> {
        let block = &self.blocks.get(&block_id).unwrap();
        let block_coords = &block.coords;
        if block.is_vertical() {
            println!("{} is vertical", block_id);
            let below = block_coords.iter().min_by(|a, b| a.z.cmp(&b.z)).unwrap();
            let below = *below - UNIT_Z;
            println!("{:?} bleow: {:?}", below, self[below]);
            if let Some(bid) = self[below] {
                HashSet::from([bid])
            } else {
                HashSet::new()
            }
        } else {
            block_coords
                .iter()
                .map(|c| *c - UNIT_Z)
                .filter_map(|coord| self[coord])
                .collect()
        }
    }
}

impl Index<Coord> for State {
    type Output = Option<BlockId>;

    fn index(&self, index: Coord) -> &Self::Output {
        self.get_point(index)
    }
}

impl IndexMut<Coord> for State {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.points[index.x + index.y * self.x_dim + index.z * self.y_dim * self.x_dim]
    }
}

impl Index<BlockId> for State {
    type Output = Block;

    fn index(&self, index: BlockId) -> &Self::Output {
        self.blocks.get(&index).unwrap()
    }
}

impl IndexMut<BlockId> for State {
    fn index_mut(&mut self, index: BlockId) -> &mut Self::Output {
        self.blocks.get_mut(&index).unwrap()
    }
}

fn q1(input: &str) -> Result<u128, String> {
    let blocks = input
        .lines()
        .enumerate()
        .map(|(idx, l)| (idx, Block::new(idx, l)))
        .collect();
    let mut state = State::new(blocks);
    state.print_from_x();

    let n = state.blocks.len();
    loop {
        let mut dropped = false;
        for block_id in 0..n {
            if state.is_droppable(state.blocks.get(&block_id).unwrap()) {
                dropped = true;
                let block = state.blocks.get(&block_id).unwrap().clone();
                for coord in block.coords.iter() {
                    state[*coord] = None;
                }
                for coord in block.coords.iter() {
                    state[*coord - UNIT_Z] = Some(block.id);
                }
                let block = state.blocks.get_mut(&block_id).unwrap();
                block
                    .coords
                    .iter_mut()
                    .for_each(|mut coord| *coord -= UNIT_Z);
            }
        }
        println!();
        state.print_from_x();
        if !dropped {
            break;
        }
    }

    let supps = (0..n).map(|x| state.supported_by(x)).collect_vec();
    for (i, supp) in supps.iter().enumerate() {
        println!("Block {}:\t{:?}", i, supp);
    }
    let mut important = HashSet::new();
    for (i, supp) in supps.iter().enumerate() {
        if supp.len() == 1 {
            println!(
                "{} is supported only 1: by {}",
                i,
                supp.iter().nth(0).unwrap()
            );
            important.insert(supp.iter().nth(0).unwrap());
        }
    }
    println!("{:?} LEN: {}", important, important.len());

    println!("Q1: {}", (n - important.len()) as u128);
    let support_vec = (0..n).map(|x| state.supports(x)).collect_vec();
    let mut q2_map: Vec<Option<usize>> = vec![None; n];
    for id in 0..n {
        if q2_map[id].is_none() {
            let mut work_list = support_vec[id].iter().collect_vec();
            while let Some(curr) = work_list.pop() {}
        }
    }

    Ok((n - important.len()) as u128)
}

fn rec_q2(
    block_id: BlockId,
    support_vec: &Vec<HashSet<BlockId>>,
    memo: &mut Vec<Option<u128>>,
) -> u128 {
    if memo[block_id].is_none() {
        let n = support_vec[block_id]
            .iter()
            .map(|bid| rec_q2(*bid, &support_vec, memo))
            .sum();
        memo[block_id] = Some(n);
        n
    } else {
        memo[block_id].unwrap()
    }
}

fn q2(input: &str) -> Result<u128, String> {
    todo!()
}
