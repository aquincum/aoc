use crate::common::day::{Day, Question};

pub struct Day7;

impl Day for Day7 {
    fn question(&self, input: &str, question: Question) {
        run_question(input);
    }

    fn test_data(&self) -> String {
        return "- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)"
            .to_string();
    }
}

#[derive(Debug)]
enum Entry {
    EntryFile(File),
    EntryDirectory(Directory),
}

use itertools::Itertools;
use Entry::EntryDirectory;
use Entry::EntryFile;

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct Directory {
    name: String,
    entries: Vec<Entry>,
    calculated_size: Option<u32>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            entries: Vec::new(),
            calculated_size: None,
        }
    }
    fn push(&mut self, e: Entry) {
        self.entries.push(e);
    }
    fn push_to(&mut self, e: Entry, path: &str) {
        if path == "/" || path == "" {
            self.push(e);
            return;
        }
        let sub_dir = path.split("/").skip(1).take(1).join("");
        let rest = format!("/{}", path.split("/").skip(2).join("/"));
        println!("A {} {} {:?}", sub_dir, rest, e);
        let d = self
            .entries
            .iter_mut()
            .map(|e| {
                if let EntryDirectory(d) = e {
                    if d.name == sub_dir {
                        return Some(d);
                    }
                }
                None
            })
            .find(Option::is_some)
            .unwrap()
            .unwrap();
        d.push_to(e, &rest);
    }
    fn calculate_size(&mut self) -> u32 {
        if let Some(n) = self.calculated_size {
            return n;
        }
        let mut sum = 0;
        for e in self.entries.iter_mut() {
            sum += match e {
                EntryFile(f) => f.size,
                EntryDirectory(d) => d.calculate_size(),
            }
        }
        self.calculated_size = Some(sum);
        println!("{}", sum);
        sum
    }
    fn collect_filter(&self, pred: &impl Fn(u32) -> bool) -> Vec<u32> {
        let mut result = Vec::new();
        if pred(self.calculated_size.unwrap()) {
            result.push(self.calculated_size.unwrap());
        }
        for e in self.entries.iter() {
            if let EntryDirectory(dir) = e {
                result.append(&mut dir.collect_filter(pred));
            }
        }
        result
    }
}

pub fn run_question(input: &str) {
    let mut root = Directory::new("/");
    let mut curr_path = "/".to_string();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let newcwdname = line.strip_prefix("$ cd ").unwrap();
            if newcwdname == "/" {
                curr_path = "/".to_string();
            } else if newcwdname == ".." {
                println!("PRE CD.. {}", curr_path);
                let parts = curr_path.split("/").collect::<Vec<_>>();
                curr_path = parts.iter().take(parts.len() - 1).join("/");
            } else {
                curr_path = format!(
                    "{}/{}",
                    if curr_path == "/" { "" } else { &curr_path },
                    newcwdname
                )
            }
            println!("new path: {}", curr_path)
        // } else {
        //     // let mut cwd_ref= cwd.entries.iter().find(|i| {
        //     //     match i {
        //     //         EntryDirectory(d) => d.name == newcwdname,
        //     //         EntryFile(_) => false
        //     //     }
        //     // }).map(|d| {
        //     //     match d {
        //     //         EntryDirectory(d) => Some(d),
        //     //         _ => None
        //     //     }
        //     // }).flatten().as_mut().unwrap();
        //     for e in cwd.entries.iter_mut() {
        //         if let EntryDirectory(dir) = e {
        //             cwd = dir;
        //             break;
        //         }
        //     }
        // }
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            let new_dir_name = line.strip_prefix("dir ").unwrap();
            let new_dir = Directory::new(new_dir_name);
            root.push_to(EntryDirectory(new_dir), &curr_path);
        } else {
            println!("{}", line);
            let parts: Vec<_> = line.split(" ").collect();
            let size = parts[0].parse().unwrap();
            let f = File {
                name: parts[1].to_string(),
                size,
            };
            root.push_to(EntryFile(f), &curr_path);
        }
    }
    root.calculate_size();
    let sum = root.collect_filter(&|n| n <= 100000);
    let free_now = 70000000 - root.calculated_size.unwrap();
    let need = 30000000 - free_now;
    let best = root.collect_filter(&|n| n >= need);
    let best = best.into_iter().min();
    println!("{}", sum.iter().sum::<u32>());
    println!("{} {} {:?}", free_now, need, best);
}
