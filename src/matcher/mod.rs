pub mod trie;
pub mod pattern;
pub mod result;
pub mod matcher;
pub mod factory;
pub mod pattern_loader;
pub mod suite;
pub mod compiled_pattern;

pub use self::pattern::Pattern;
pub use self::matcher::Matcher;
pub use self::pattern_loader::PatternLoader;
pub use self::factory::MatcherFactory;
pub use self::matcher::builder::Builder;
pub use self::suite::MatcherSuite;
