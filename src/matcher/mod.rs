pub mod trie;
pub mod pattern;
pub mod result;
pub mod pattern_source;
pub mod factory;
pub mod pattern_loader;
pub mod suite;
pub mod compiled_pattern;
pub mod suffix_array;

pub use self::pattern::Pattern;
pub use self::pattern_loader::PatternLoader;
pub use self::factory::MatcherFactory;
pub use self::suite::MatcherSuite;
pub use self::pattern_source::{FromPatternSource, BuildError};

use matcher::result::MatchResult;
use std::fmt;

pub trait Matcher: fmt::Debug {
    fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>>;
    fn add_pattern(&mut self, pattern: Pattern);
    fn boxed_clone(&self) -> Box<Matcher>;
}
