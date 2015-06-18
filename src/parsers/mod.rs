mod set;
mod base;
mod int;

use std::fmt::Debug;
pub use self::set::SetParser;
pub use self::base::ParserBase;
pub use self::int::IntParser;

pub trait ObjectSafeHash {
    fn hash_os(&self) -> u64;
}

pub trait Parser: Debug + ObjectSafeHash {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<(&'a str, &'b str)>;
    fn base(&self) -> &ParserBase;
    fn base_mut(&mut self) -> &mut ParserBase;
}

impl HasOptionalParameter for Parser {
    fn set_optional_params(&mut self, params: &Vec<OptionalParameter>) -> bool {
        self.base_mut().set_optinal_params(params)
    }
}

pub trait HasOptionalParameter {
    fn set_optional_params(&mut self, params: &Vec<OptionalParameter>) -> bool;
}

pub enum OptionalParameter<'a> {
    Int(&'a str, u64),
    Str(&'a str, &'a str)
}
