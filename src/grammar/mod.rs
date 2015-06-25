use parsers::{OptionalParameter, HasOptionalParameter};

mod pattern_parser;
#[cfg(test)]
mod test;

pub mod parser {
    pub use super::pattern_parser::pattern;
}

pub fn unescape_literal(literal: &str) -> String {
      literal.replace(r#"\%\{"#, "%{")
}

pub fn set_optional_params<'a, T: HasOptionalParameter>(parser: &mut T, params: Option<&Vec<OptionalParameter<'a>>>) {
    if let Some(optional_params) = params {
      parser.set_optional_params(&optional_params);
    }
}
