use std::collections::{HashMap, VecDeque};

pub fn question(input: &str, msg_length: usize) -> Option<usize> {
    let mut last_four = VecDeque::new();
    for (i, ch) in input.chars().enumerate() {
        last_four.push_back(ch);
        if last_four.len() > msg_length {
            last_four.pop_front();
        }
        if last_four.len() == msg_length {
            //goodbye functional programming :'(
            let mut letter_map = HashMap::new();
            for ch in last_four.iter() {
                let entry = letter_map.entry(ch).or_insert(0 as usize);
                *entry += 1;
            }
            if !letter_map.values().any(|n| *n > 1) {
                return Some(i + 1);
            }
        }
    }

    None
}
