mod pattern_parser;
#[cfg(test)]
mod test;

pub fn unescape_literal(literal: &str) -> String {
      literal.replace(r#"\%\{"#, "%{")
}
