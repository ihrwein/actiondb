use parsers::Parser;

mod pattern_parser;
#[cfg(test)]
mod test;

pub fn unescape_literal(literal: &str) -> String {
      literal.replace(r#"\%\{"#, "%{")
}

pub type BoxedParser<'a> = Box<Parser + 'a>;
