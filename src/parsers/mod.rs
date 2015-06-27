mod set;
mod base;
mod int;
mod length_checked;

use std::fmt::Debug;
pub use self::set::SetParser;
pub use self::base::ParserBase;
pub use self::int::IntParser;
pub use self::length_checked::LengthCheckedParserBase;

pub trait ObjectSafeHash {
    fn hash_os(&self) -> u64;
}

pub trait Parser: Debug + ObjectSafeHash {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<(&'a str, &'b str)>;
    fn name(&self) -> &str;
}


pub trait HasOptionalParameter {
    fn set_optional_params<'a>(&mut self, params: &Vec<OptionalParameter<'a>>) -> bool;
}

#[derive(Debug)]
pub enum OptionalParameter<'a> {
    Int(&'a str, usize),
}
