mod set;

pub use self::set::SetParser;

pub enum MatchResult {
    Matched(usize),
    NotMatched
}

pub trait Parser {
    fn parse(&self, value: &str) -> MatchResult;
}
