use super::pattern_parser;
use matcher::trie::node::TokenType;
use parsers::{SetParser, Parser, ObjectSafeHash, IntParser, GreedyParser};

fn assert_parser_name_equals(item: Option<&TokenType>, expected_name: &str) {
    if let Some(&TokenType::Parser(ref parser)) = item {
        assert_eq!(parser.name(), expected_name);
    } else {
        unreachable!();
    }
}

fn assert_parser_equals(got: Option<&TokenType>, expected: &Parser) {
    if let Some(&TokenType::Parser(ref parser)) = got {
        println!("expected: {:?}", expected);
        println!("got: {:?}", parser);
        assert_eq!(parser.hash_os(), expected.hash_os());
    } else {
        unreachable!();
    }
}

fn assert_literal_equals(item: Option<&TokenType>, expected: &str) {
    if let Some(&TokenType::Literal(ref literal)) = item {
        assert_eq!(literal, expected);
    } else {
        unreachable!();
    }
}

#[test]
fn test_given_parser_as_a_string_when_it_is_parsed_then_we_get_the_instantiated_parser() {
    let string_parser = "%{INT:test_name}";
    let vec = pattern_parser::pattern(string_parser).ok().unwrap();

    assert_eq!(vec.len(), 1);
    println!("{:?}", &vec);
    assert_parser_name_equals(vec.get(0), "test_name");
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
    let vec = pattern_parser::pattern(expected).ok().unwrap();

    assert_eq!(vec.len(), 1);

    assert_literal_equals(vec.get(0), expected);
}

#[test]
fn test_given_pattern_as_a_string_when_it_is_parsed_with_the_grammar_we_got_the_right_compiled_pattern() {
    let pattern_as_string = "foo %{INT:int_0} bar %{INT:int_1}%{INT:int_2} baz";
    let vec: Vec<TokenType<>> = pattern_parser::pattern(pattern_as_string).ok().unwrap();

    assert_eq!(vec.len(), 6);
    assert_literal_equals(vec.get(0), "foo ");
    assert_parser_name_equals(vec.get(1), "int_0");
    assert_literal_equals(vec.get(2), " bar ");
    assert_parser_name_equals(vec.get(3), "int_1");
    assert_parser_name_equals(vec.get(4), "int_2");
    assert_literal_equals(vec.get(5), " baz");
}

#[test]
#[should_panic]
fn test_given_invalid_string_when_we_parse_it_then_the_parser_returns_with_error() {
    let pattern_as_string = "foo %{INT:int_0 baz";
    let _ = pattern_parser::pattern(pattern_as_string).ok().unwrap();
}

#[test]
fn test_given_string_which_contains_escaped_chars_when_we_parse_it_then_we_get_the_right_string() {
    let vec = pattern_parser::pattern(r#"foo \%\{ %{INT:test_name} baz"#).ok().unwrap();
    assert_eq!(vec.len(), 3);
    assert_literal_equals(vec.get(0), "foo %{ ");
    assert_parser_name_equals(vec.get(1), "test_name");
    assert_literal_equals(vec.get(2), " baz");
}

#[test]
fn test_given_set_parser_with_character_set_parameter_when_we_parse_it_then_we_get_the_right_parser() {
    let expected_parser = SetParser::from_str("test_set", "0123456789");
    let vec = pattern_parser::pattern(r#"%{SET("0123456789"):test_set}"#).ok().unwrap();
    assert_eq!(vec.len(), 1);
    assert_parser_equals(vec.get(0), &expected_parser);
}

#[test]
fn test_given_set_parser_with_optional_parameters_when_we_parse_it_then_we_get_the_right_parser() {
    let mut expected_parser = SetParser::from_str("test_set", "0123456789");
    expected_parser.set_min_length(2);
    expected_parser.set_max_length(5);

    let vec = pattern_parser::pattern(r#"%{SET("0123456789",min_len=2, max_len=5):test_set}"#).ok().unwrap();
    assert_eq!(vec.len(), 1);
    assert_parser_equals(vec.get(0), &expected_parser);
}

#[test]
fn test_given_int_parser_with_optional_parameters_when_we_parse_it_then_we_get_the_right_parser() {
    let mut expected_parser = IntParser::from_str("test_int");
    expected_parser.set_min_length(2);
    expected_parser.set_max_length(5);

    let vec = pattern_parser::pattern(r#"%{INT(min_len=2,max_len=5):test_int}"#).ok().unwrap();
    assert_eq!(vec.len(), 1);
    assert_parser_equals(vec.get(0), &expected_parser);
}

#[test]
fn test_given_greedy_parser_when_we_parse_it_then_we_get_the_right_result() {
    let expected_parser = GreedyParser::from_str("greedy", " baz");
    let pattern_as_string = "foo %{INT:int_0} bar %{GREEDY:greedy} baz";
    let vec: Vec<TokenType<>> = pattern_parser::pattern(pattern_as_string).ok().unwrap();

    assert_eq!(vec.len(), 5);
    assert_literal_equals(vec.get(0), "foo ");
    assert_parser_name_equals(vec.get(1), "int_0");
    assert_literal_equals(vec.get(2), " bar ");
    assert_parser_name_equals(vec.get(3), "greedy");
    assert_literal_equals(vec.get(4), " baz");
    assert_parser_equals(vec.get(3), &expected_parser);
}

#[test]
fn test_given_greedy_parser_when_there_is_no_literal_after_it_then_we_take_all_the_remaining_intput_as_matching() {
    let pattern_as_string = "bar %{GREEDY:greedy}";
    let vec: Vec<TokenType<>> = pattern_parser::pattern(pattern_as_string).ok().unwrap();

    if let &TokenType::Parser(ref parser) = vec.get(1).unwrap() {
        assert_eq!(parser.parse("the quick brown fox"), Some(("greedy", "the quick brown fox")));
    } else {
        unreachable!();
    }
}

#[test]
fn test_given_parser_when_there_is_a_dot_in_its_name_then_it_is_ok() {
    let pattern_as_string = "bar %{GREEDY:.some.dotted_notation}";
    let vec: Vec<TokenType<>> = pattern_parser::pattern(pattern_as_string).ok().unwrap();
    assert_parser_name_equals(vec.get(1), ".some.dotted_notation");
}
