pub mod pattern_parser;
#[cfg(test)]
mod test;

pub mod parser {
    pub use super::pattern_parser::pattern;
    pub use super::pattern_parser::ParseError;
}

pub fn unescape_literal(literal: &str) -> String {
      literal.replace(r#"\%\{"#, "%{")
}
