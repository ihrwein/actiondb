mod set;
mod base;
mod int;
mod factory;

use std::fmt::Debug;
pub use self::set::SetParser;
pub use self::base::ParserBase;
pub use self::int::IntParser;
pub use self::factory::ParserFactory;

pub trait ObjectSafeHash {
    fn hash_os(&self) -> u64;
}

pub trait Parser: Debug + ObjectSafeHash {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<(&'a str, &'b str)>;
    fn base(&self) -> &ParserBase;
    fn base_mut(&mut self) -> &mut ParserBase;
}

impl<T> HasOptionalParameter for T where T:Parser {
    fn set_optional_params<'a>(&mut self, params: &Vec<OptionalParameter<'a>>) -> bool {
        self.base_mut().set_optional_params(params)
    }
}

pub trait HasOptionalParameter {
    fn set_optional_params<'a>(&mut self, params: &Vec<OptionalParameter<'a>>) -> bool;
}

#[derive(Debug)]
pub enum OptionalParameter<'a> {
    Int(&'a str, usize),
    Str(&'a str, &'a str)
}
