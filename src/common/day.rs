#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Question {
    First,
    Second,
}

pub trait Day {
    fn question(&self, input: &str, question: Question);
    fn test_data(&self) -> String;
}
