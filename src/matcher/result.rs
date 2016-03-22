use std::collections::BTreeMap;

use parsers::ParseResult;
use matcher::Pattern;

#[derive(Debug)]
pub struct MatchResult<'a, 'b> {
    pattern: &'a Pattern,
    values: BTreeMap<&'a str, &'b str>,
}

impl <'a, 'b> MatchResult<'a, 'b> {
    pub fn new(pattern: &'a Pattern) -> MatchResult<'a, 'b> {
        MatchResult {
            pattern: pattern,
            values: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, result: ParseResult<'a, 'b>) {
        if let Some(name) = result.parser().name() {
            self.values.insert(name, result.value());
        }
    }

    pub fn pattern(&self) -> &Pattern {
        self.pattern
    }

    pub fn values(&self) -> &BTreeMap<&'a str, &'b str> {
        &self.values
    }
}

#[cfg(test)]
mod test {
    use parsers::{IntParser, ParseResult};
    use super::MatchResult;
    use matcher::Pattern;
    use std::collections::BTreeMap;

    #[test]
    fn test_given_match_result_when_a_parse_result_is_inserted_then_we_use_only_the_ones_where_the_parser_has_a_name
        () {
        let parser_wo_name = IntParser::new();
        let parser_with_name = IntParser::with_name("name".to_owned());
        let expected_values = {
            let mut map = BTreeMap::new();
            map.insert("name", "c");
            map
        };
        let pattern = Pattern::with_random_uuid();
        let mut match_result = MatchResult::new(&pattern);
        match_result.insert(ParseResult::new(&parser_wo_name, "a"));
        match_result.insert(ParseResult::new(&parser_wo_name, "b"));
        match_result.insert(ParseResult::new(&parser_with_name, "c"));
        assert_eq!(match_result.values(), &expected_values);
    }
}
