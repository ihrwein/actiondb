use parsers::{GreedyParser, IntParser, OptionalParameter, Parser, ParserFactory, SetParser,
              HasLengthConstraint};

macro_rules! set_optinal_param {
    ($parser:expr, $param:expr) => {
        match $param {
            OptionalParameter::Int(key, value) => {
                match key {
                    "min_len" => {
                        $parser.set_min_length(Some(value));
                    },
                    "max_len" => {
                        $parser.set_max_length(Some(value));
                    },
                    _ => ()
                }
            }
        }
    }
}

macro_rules! set_optional_params {
    ($parser:expr, $opt_params:expr) => {
        if let Some(opt_params) = $opt_params {
            for i in opt_params.into_iter() {
                set_optinal_param!($parser, i);
            }
        }
    }
}

pub struct TrieParserFactory;

impl ParserFactory for TrieParserFactory {
    fn new_set<'a>(set: &str,
                   name: Option<&str>,
                   opt_params: Option<Vec<OptionalParameter<'a>>>)
                   -> Box<Parser> {
        let mut parser = SetParser::new(set);
        set_optional_params!(&mut parser, opt_params);
        let name = name.map(|name| name.to_owned());
        parser.set_name(name);
        Box::new(parser)
    }
    fn new_int(name: Option<&str>,
                   opt_params: Option<Vec<OptionalParameter>>)
                   -> Box<Parser> {
        let mut parser = IntParser::new();
        set_optional_params!(&mut parser, opt_params);
        let name = name.map(|name| name.to_owned());
        parser.set_name(name);
        Box::new(parser)
    }
    fn new_greedy(name: Option<&str>, end_string: Option<&str>) -> Box<Parser> {
        let mut parser = GreedyParser::new();
        let end_string = end_string.map(|string| string.to_owned());
        parser.set_end_string(end_string);
        let name = name.map(|name| name.to_owned());
        parser.set_name(name);
        Box::new(parser)
    }
}
