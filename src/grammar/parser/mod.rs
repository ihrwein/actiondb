pub use self::pattern_parser::ParseError;

use matcher::compiled_pattern::CompiledPattern;
use parsers::ParserFactory;
use self::pattern_parser::ParseResult;

#[allow(clippy)]
mod pattern_parser;

pub fn pattern_with_factory<F: ParserFactory>(input: &str) -> ParseResult<CompiledPattern> {
    self::pattern_parser::pattern::<F>(input)
}

//
// When you regenerate the grammar, don't forget to insert the F generic type
// parameter with
// the following command:
// rm -f pattern_parser.rs; cat pattern_parser.rs.IN | sed  "s/\(fn
// [a-zA-Z0-9_]*<'input\)/\1, F: ParserFactory/" | sed
// "s/\(parse[a-zA-Z0-9_]*\)(/\1::<F>(/" >> pattern_parser.rs
//
// The first sed add the F: ParserFactory generic type parameter to every
// function definition.
// The second sed threads this F parameter through the call sites as well.
//
pub fn pattern(input: &str) -> ParseResult<CompiledPattern> {
    use matcher::trie::parser_factory::TrieParserFactory;
    self::pattern_parser::pattern::<TrieParserFactory>(input)
}
