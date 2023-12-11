pub struct Columns {
    s: String,
    pos: usize,
    width: usize,
}

impl Iterator for Columns {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.s.len();
        if self.pos == self.width {
            None
        } else {
            let mut s = String::new();
            while self.pos < len {
                let ch = self.s.as_bytes()[self.pos];
                s.push(ch as char);
                self.pos += self.width;
            }
            self.pos -= len - 1;
            Some(s)
        }
    }
}

pub trait Columnser {
    fn columns(&self, width: usize) -> Columns;
}

impl Columnser for str {
    fn columns(&self, width: usize) -> Columns {
        Columns {
            s: self.replace("\n", ""),
            pos: 0,
            width,
        }
    }
}
