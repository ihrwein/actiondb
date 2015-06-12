use super::pattern_parser;
use matcher::trie::node::{CompiledPattern, NodeType};

#[test]
fn test_given_parser_as_a_string_when_it_is_parsed_then_we_get_the_instantiated_parser() {
    let string_parser = "%{SET:test_name}";
    match pattern_parser::part_parser(string_parser).ok().unwrap() {
        NodeType::Parser(parser) => {
            assert_eq!(parser.name(), "test_name");
        },
        _ => unreachable!()
    }
}

#[test]
fn test_given_parser_as_a_string_when_its_name_is_invalid_then_we_dont_get_the_instantiated_parser() {
    pattern_parser::part_parser("%{SET:test$name}").err().unwrap();
    pattern_parser::part_parser("%{SET:test-name}").err().unwrap();
    pattern_parser::part_parser("%{SET:-").err().unwrap();
    pattern_parser::part_parser("%{SET:").err().unwrap();
}

#[test]
fn test_given_parser_as_a_string_when_its_name_is_valid_then_we_get_the_instantiated_parser() {
    pattern_parser::part_parser("%{SET:test_name}").ok().unwrap();
    pattern_parser::part_parser("%{SET:test}").ok().unwrap();
    pattern_parser::part_parser("%{SET:TEST_NAME_}").ok().unwrap();
    pattern_parser::part_parser("%{SET:_}").ok().unwrap();
}

#[test]
fn test_given_parser_as_a_string_when_its_type_isnt_exist_then_we_get_an_error() {
    let string_parser_with_invalid_type = "%{INVALID:test_name}";
    pattern_parser::part_parser(string_parser_with_invalid_type).err().unwrap();
}
