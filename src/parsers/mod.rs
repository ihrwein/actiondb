mod set;
mod base;

use std::fmt::Debug;
pub use self::set::SetParser;
pub use self::base::ParserBase;

pub trait ObjectSafeHash {
    fn hash_os(&self) -> u64;
}

pub trait Parser: Debug + ObjectSafeHash {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<(&'a str, &'b str)>;
    fn base(&self) -> &ParserBase;
    fn base_mut(&mut self) -> &mut ParserBase;
}
