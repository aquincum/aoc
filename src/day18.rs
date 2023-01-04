use crate::common::day::{Day, Question};

pub struct Day18;
impl Day for Day18 {
    fn question(&self, input: &str, question: Question) {
        if question == Question::First {
            bruteforce::q1(input)
        } else {
            todo!("Not solved yet")
        }
    }

    fn test_data(&self) -> String {
        "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            .to_string()
    }
}

mod bruteforce {
    use std::num::ParseIntError;
    use std::str::FromStr;

    #[derive(Copy, Clone)]
    struct Cube {
        x: u32,
        y: u32,
        z: u32,
        a_left: bool,
        a_right: bool,
        a_top: bool,
        a_bottom: bool,
        a_front: bool,
        a_back: bool,
    }

    impl Cube {
        fn new(x: u32, y: u32, z: u32) -> Self {
            Cube {
                x,
                y,
                z,
                a_left: false,
                a_right: false,
                a_top: false,
                a_bottom: false,
                a_front: false,
                a_back: false,
            }
        }
        fn process_neighbor(&mut self, neighbor: &Cube) {
            if neighbor.x + 1 == self.x && neighbor.y == self.y && neighbor.z == self.z {
                self.a_left = true;
            } else if neighbor.x == self.x + 1 && neighbor.y == self.y && neighbor.z == self.z {
                self.a_right = true;
            } else if neighbor.x == self.x && neighbor.y + 1 == self.y && neighbor.z == self.z {
                self.a_bottom = true;
            } else if neighbor.x == self.x && neighbor.y == self.y + 1 && neighbor.z == self.z {
                self.a_top = true;
            } else if neighbor.x == self.x && neighbor.y == self.y && neighbor.z + 1 == self.z {
                self.a_front = true;
            } else if neighbor.x == self.x && neighbor.y == self.y && neighbor.z == self.z + 1 {
                self.a_back = true;
            }
        }
        fn sides(&self) -> u32 {
            let occupied: u32 = <bool as Into<u32>>::into(self.a_left)
                + <bool as Into<u32>>::into(self.a_right)
                + <bool as Into<u32>>::into(self.a_bottom)
                + <bool as Into<u32>>::into(self.a_top)
                + <bool as Into<u32>>::into(self.a_front)
                + <bool as Into<u32>>::into(self.a_back);
            6 - occupied
        }
    }

    impl FromStr for Cube {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let nums = s.split(",").collect::<Vec<_>>();
            if nums.len() != 3 {
                return Err("no 3 numbers".to_string());
            }
            let x = nums[0].parse().map_err(|e: ParseIntError| e.to_string())?;
            let y = nums[1].parse().map_err(|e: ParseIntError| e.to_string())?;
            let z = nums[2].parse().map_err(|e: ParseIntError| e.to_string())?;
            Ok(Cube::new(x, y, z))
        }
    }

    pub fn q1(input: &str) {
        let cubes: Result<Vec<Cube>, _> = input.split("\n").map(|l| l.parse()).collect();
        let mut cubes = cubes.unwrap();
        for i in 0..cubes.len() {
            for j in 0..cubes.len() {
                let c = cubes[j];
                cubes[i].process_neighbor(&c);
            }
        }
        let sum: u32 = cubes.iter().map(|c| c.sides()).sum();
        println!("{}", sum);
    }
}
