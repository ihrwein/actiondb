mod set;

use std::fmt::Debug;
pub use self::set::SetParser;

pub trait ObjectSafeHash {
    fn hash_os(&self) -> u64;
}

pub trait Parser: Debug + ObjectSafeHash {
    fn parse<'a>(&self, value: &'a str) -> Option<&'a str>;
}
