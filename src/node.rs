use std::collections::BTreeMap;
use std::borrow::ToOwned;

use parsers::{Parser, SetParser};

type MatchResult<'a> = Option<BTreeMap<&'a str, &'a str>>;
type CompiledPattern<'a, 'b> = Vec<NodeType<'a, 'b>>;

enum NodeType<'a, 'b> {
    Parser(Box<Parser<'a, 'b>>),
    Literal(String)
}
