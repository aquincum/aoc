pub fn q1(input: String) -> i32 {
    let elf_calories = get_elves(&input);
    let max_elf: Option<i32> = elf_calories.max();
    max_elf.unwrap_or(-1)
}

pub fn q2(input: String) -> i32 {
    let mut elf_calories = get_elves(&input).collect::<Vec<_>>();
    elf_calories.sort_unstable();
    elf_calories.into_iter().rev().take(3).sum()
}

pub fn q2_alt(input: String) -> i32 {
    let elf_calories = get_elves(&input);
    struct Top3 {
        fst: i32,
        sec: i32,
        thr: i32,
    }
    impl Top3 {
        fn sum(self) -> i32 {
            self.fst + self.sec + self.thr
        }
    }
    elf_calories
        .fold(
            Top3 {
                fst: 0,
                sec: 0,
                thr: 0,
            },
            |top3, elf| match (elf > top3.fst, elf > top3.sec, elf > top3.thr) {
                (true, _, _) => Top3 {
                    fst: elf,
                    sec: top3.fst,
                    thr: top3.sec,
                },
                (false, true, _) => Top3 {
                    fst: top3.fst,
                    sec: elf,
                    thr: top3.sec,
                },
                (false, false, true) => Top3 {
                    fst: top3.fst,
                    sec: top3.sec,
                    thr: elf,
                },
                (false, false, false) => top3,
            },
        )
        .sum()
}

fn get_elves<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a {
    let elves = input.split("\n\n");
    let foods = elves.map(|elf| elf.split("\n").filter(|s| !s.is_empty()));
    let elf_calories = foods.map(|lines| {
        lines
            .map(|line| {
                line.parse::<i32>()
                    .map_err(|e| {
                        println!("Yo what {} {}", line, e);
                    })
                    .unwrap()
            })
            .sum()
    });
    elf_calories
}
