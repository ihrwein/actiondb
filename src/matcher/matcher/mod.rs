use super::result::MatchResult;
use super::pattern::Pattern;
use std::fmt;

pub mod builder;

pub trait Matcher: fmt::Debug {
    fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>>;
    fn add_pattern(&mut self, pattern: Pattern);
}
