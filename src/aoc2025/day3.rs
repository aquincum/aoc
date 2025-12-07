use crate::common::day::{Day, Question};

pub struct Day3;

impl Day for Day3 {
    fn question(&self, input: &str, question: Question) {
        let banks = input.lines().map(|l| Banks::new(l));
        let max_nums = banks.map(|b| match question {
            Question::First => b.q1(),
            Question::Second => b.q2(12usize),
        });
        let max_nums = max_nums.map(|n| {
            println!("{}", n);
            n
        });
        let s = max_nums.sum::<u128>();
        println!("{}", s);
    }

    fn test_data(&self) -> String {
        "987654321111111
811111111111119
234234234234278
818181911112111"
            .to_string()
    }
}

struct Banks(Vec<usize>);

fn first_max<'a, I>(iter: I, len: usize, min: usize) -> (usize, usize)
where
    I: Iterator<Item = &'a usize>,
{
    iter.enumerate()
        .take(len - min)
        .fold((0usize, 0usize), |(acci, accn), (i, n)| {
            if *n > accn {
                (i, *n)
            } else {
                (acci, accn)
            }
        })
}

impl Banks {
    fn new(input: &str) -> Self {
        Self(
            input
                .lines()
                .next()
                .unwrap()
                .chars()
                .map(|s| s.to_digit(10).unwrap() as usize)
                .collect(),
        )
    }
    fn q1(&self) -> u128 {
        let first_max = first_max(self.0.iter(), self.0.len(), 1);
        let second_max = self.0.iter().skip(first_max.0 + 1).max().unwrap();
        (first_max.1 * 10 + second_max) as u128
    }

    fn q2(&self, digits: usize) -> u128 {
        let mut lastidx = None;
        let mut num = vec![];
        let len = self.0.len();
        for i in 0..digits {
            let fistidxtocheck = lastidx.map(|n| n + 1).unwrap_or(0usize);
            let (idx, val) = first_max(
                self.0.iter().skip(fistidxtocheck),
                len - fistidxtocheck,
                digits - i - 1,
            );
            println!("{} {} {}", idx, val, self.0[idx + fistidxtocheck]);
            lastidx = Some(idx + fistidxtocheck);
            num.push(val);
        }
        num.into_iter()
            .enumerate()
            .map(|(d, n)| 10u128.pow((digits - d - 1) as u32) * n as u128)
            .sum()
    }
}
