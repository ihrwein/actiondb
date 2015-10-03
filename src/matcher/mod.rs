pub mod trie;
pub mod pattern;
pub mod result;
pub mod matcher;
pub mod factory;
pub mod compiled_pattern;

pub use self::pattern::Pattern;
pub use self::matcher::Matcher;
pub use self::factory::Factory;
pub use self::factory::MatcherFactory;
