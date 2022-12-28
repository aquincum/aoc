use std::str::Split;

pub trait Lineser {
    fn lines(&self) -> Split<&str>;
}

impl Lineser for str {
    fn lines(&self) -> Split<&str> {
        self.split("\n")
    }
}
