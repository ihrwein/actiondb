use parsers::{Parser, OptionalParameter, HasOptionalParameter};
use matcher::trie::node::NodeType;

mod pattern_parser;
#[cfg(test)]
mod test;

pub fn unescape_literal(literal: &str) -> String {
      literal.replace(r#"\%\{"#, "%{")
}

pub fn set_optional_params<'a, T: HasOptionalParameter>(parser: &mut T, params: Option<&Vec<OptionalParameter<'a>>>) {
    if let Some(optional_params) = params {
      parser.set_optional_params(&optional_params);
    }
}

pub fn set_name_and_wrap_into(mut parser: Box<Parser>, name: &str) -> NodeType {
    parser.base_mut().set_name(name.to_string());
    NodeType::Parser(parser)
}
