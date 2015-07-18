pub mod trie;
pub mod pattern;
pub mod result;
pub mod matcher;
mod errors;

pub use self::pattern::Pattern;
pub use self::matcher::Matcher;
