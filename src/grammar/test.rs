use super::pattern_parser;
use matcher::trie::node::{CompiledPattern, NodeType};

#[test]
fn test_given_parser_as_a_string_when_it_is_parsed_then_we_get_the_instantiated_parser() {
    let string_parser = "%{INT:test_name}";

    let mut vec = pattern_parser::pattern(string_parser).ok().unwrap();

    assert_eq!(vec.len(), 1);
    println!("{:?}", &vec);

    if let Some(NodeType::Parser(parser)) = vec.pop() {
        assert_eq!(parser.base().name(), "test_name");
    } else {
        unreachable!();
    }
}

#[test]
fn test_given_parser_as_a_string_when_its_name_is_invalid_then_we_dont_get_the_instantiated_parser() {
    pattern_parser::pattern("%{INT:test$name}").err().unwrap();
    pattern_parser::pattern("%{INT:test-name}").err().unwrap();
    pattern_parser::pattern("%{INT:-").err().unwrap();
    pattern_parser::pattern("%{INT:").err().unwrap();
}

#[test]
fn test_given_parser_as_a_string_when_its_name_is_valid_then_we_get_the_instantiated_parser() {
    pattern_parser::pattern("%{INT:test_name}").ok().unwrap();
    pattern_parser::pattern("%{INT:test}").ok().unwrap();
    pattern_parser::pattern("%{INT:TEST_NAME_}").ok().unwrap();
    pattern_parser::pattern("%{INT:_}").ok().unwrap();
}

#[test]
fn test_given_parser_as_a_string_when_its_type_isnt_exist_then_we_get_an_error() {
    let string_parser_with_invalid_type = "%{INVALID:test_name}";
    pattern_parser::pattern(string_parser_with_invalid_type).err().unwrap();
}


#[test]
fn test_given_literal_as_a_string_when_it_is_parsed_then_we_stop_at_the_parsers_begin() {
    let expected = "foo ";
    let mut vec = pattern_parser::pattern(expected).ok().unwrap();

    assert_eq!(vec.len(), 1);

    if let Some(NodeType::Literal(literal)) = vec.pop() {
        assert_eq!(literal, expected);
    } else {
        unreachable!();
    }
}
/*#[test]
fn test_given_pattern_as_a_string_when_it_is_parsed_with_the_grammar_we_got_the_right_compiled_pattern() {
    //let pattern_as_string = "foo %{INT:int_0} bar %{INT:int_1} %{INT:int_2} baz";
    let pattern_as_string = "foo ";
    let res = pattern_parser::pattern(pattern_as_string);
    //println!("{:?}", &res);
    res.ok().unwrap();
}*/
