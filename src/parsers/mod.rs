mod set;

pub use self::set::SetParser;

pub enum MatchResult<'a> {
    Matched(&'a str),
    NotMatched
}

pub trait Parser<'s, 'a> {
    fn parse(&'s self, value: &'a str) -> MatchResult<'a>;
}
