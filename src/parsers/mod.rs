mod set;

pub use self::set::SetParser;

#[derive(PartialEq, Debug)]
pub enum ParseResult<'a> {
    Parsed(&'a str),
    NotParsed
}

pub trait Parser<'a> {
    fn parse(&self, value: &'a str) -> ParseResult<'a>;
}
