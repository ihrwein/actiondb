#[cfg(test)]
mod test;
pub mod parser;

pub fn unescape_literal(literal: &str) -> String {
    literal.replace(r#"\%\{"#, "%{")
}
