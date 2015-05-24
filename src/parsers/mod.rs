mod set;

pub use self::set::SetParser;

#[derive(PartialEq, Debug)]
pub enum MatchResult<'a> {
    Matched(&'a str),
    NotMatched
}

pub trait Parser<'a, 'b> {
    fn parse(&'a self, value: &'b str) -> MatchResult<'b>;
}
