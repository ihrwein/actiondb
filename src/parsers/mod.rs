mod set;

pub use self::set::SetParser;

#[derive(PartialEq, Debug)]
pub enum ParseResult<'a> {
    Parsed(&'a str),
    NotParsed
}

pub trait Parser<'a, 'b> {
    fn parse(&self, value: &'b str) -> ParseResult<'b>;
}
