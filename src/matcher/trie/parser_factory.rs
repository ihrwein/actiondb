use parsers::{
    GreedyParser,
    HasOptionalParameter,
    IntParser,
    OptionalParameter,
    Parser,
    ParserFactory,
    SetParser,
};

pub struct TrieParserFactory;

impl ParserFactory for TrieParserFactory {
    fn new_set<'a>(set: &str, name: Option<&str>, opt_params: Option<&Vec<OptionalParameter<'a>>>) -> Box<Parser> {
        let mut parser = SetParser::new(set);
        if let Some(params) = opt_params {
          parser.set_optional_params(params);
        }
        let name = name.map(|name| name.to_string());
        parser.set_name(name);
        Box::new(parser)
    }
    fn new_int<'a>(name: Option<&str>, opt_params: Option<&Vec<OptionalParameter<'a>>>) -> Box<Parser> {
        let mut parser = IntParser::new();
        if let Some(params) = opt_params {
          parser.set_optional_params(params);
        }
        let name = name.map(|name| name.to_string());
        parser.set_name(name);
        Box::new(parser)
    }
    fn new_greedy<'a>(name: Option<&str>, end_string: Option<&str>) -> Box<Parser> {
        let mut parser = GreedyParser::new();
        let end_string = end_string.map(|string| string.to_string());
        parser.set_end_string(end_string);

        let name = name.map(|name| name.to_string());
        parser.set_name(name);
        Box::new(parser)
    }
}
