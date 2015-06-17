mod pattern_parser;
#[cfg(test)]
mod test;

pub fn unescape_literal(literal: &str) -> &str {
      literal.replace(r#"\%\{"#, "%{")
}
