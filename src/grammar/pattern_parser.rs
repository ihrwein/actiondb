// Generated by rust-peg. Do not edit.
#![allow(non_snake_case, unused)]
use matcher::trie::node::{CompiledPattern};
use matcher::trie::node::{Node, NodeType};
use parsers::SetParser;
use self::RuleResult::{Matched, Failed};
fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}
fn char_range_at(s: &str, pos: usize) -> (char, usize) {
    let c = &s[pos..].chars().next().unwrap();
    let next_pos = pos + c.len_utf8();
    (*c, next_pos)
}
#[derive(Clone)]
enum RuleResult<T> { Matched(usize, T), Failed, }
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub expected: ::std::collections::HashSet<&'static str>,
}
pub type ParseResult<T> = Result<T, ParseError>;
impl ::std::fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter)
     -> ::std::result::Result<(), ::std::fmt::Error> {
        try!(write ! (
             fmt , "error at {}:{}: expected " , self . line , self . column
             ));
        if self.expected.len() == 1 {
            try!(write ! (
                 fmt , "`{}`" , escape_default (
                 self . expected . iter (  ) . next (  ) . unwrap (  ) ) ));
        } else {
            let mut iter = self.expected.iter();
            try!(write ! (
                 fmt , "one of `{}`" , escape_default (
                 iter . next (  ) . unwrap (  ) ) ));
            for elem in iter {
                try!(write ! ( fmt , ", `{}`" , escape_default ( elem ) ));
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for ParseError {
    fn description(&self) -> &str { "parse error" }
}
fn slice_eq(input: &str, state: &mut ParseState, pos: usize, m: &'static str)
 -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    let l = m.len();
    if input.len() >= pos + l &&
           &input.as_bytes()[pos..pos + l] == m.as_bytes() {
        Matched(pos + l, ())
    } else { state.mark_failure(pos, m) }
}
fn slice_eq_case_insensitive(input: &str, state: &mut ParseState, pos: usize,
                             m: &'static str) -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    let mut used = 0usize;
    let mut input_iter = input[pos..].chars().flat_map(|x| x.to_uppercase());
    for m_char_upper in m.chars().flat_map(|x| x.to_uppercase()) {
        used += m_char_upper.len_utf8();
        let input_char_result = input_iter.next();
        if input_char_result.is_none() ||
               input_char_result.unwrap() != m_char_upper {
            return state.mark_failure(pos, m);
        }
    }
    Matched(pos + used, ())
}
fn any_char(input: &str, state: &mut ParseState, pos: usize)
 -> RuleResult<()> {
    #![inline]
    #![allow(dead_code)]
    if input.len() > pos {
        let (_, next) = char_range_at(input, pos);
        Matched(next, ())
    } else { state.mark_failure(pos, "<character>") }
}
fn pos_to_line(input: &str, pos: usize) -> (usize, usize) {
    let mut remaining = pos;
    let mut lineno: usize = 1;
    for line in input.lines() {
        let line_length = line.len() + 1;
        if remaining < line_length { return (lineno, remaining + 1); }
        remaining -= line_length;
        lineno += 1;
    }
    return (lineno, remaining + 1);
}
struct ParseState {
    max_err_pos: usize,
    expected: ::std::collections::HashSet<&'static str>,
}
impl ParseState {
    fn new() -> ParseState {
        ParseState{max_err_pos: 0,
                   expected: ::std::collections::HashSet::new(),}
    }
    fn mark_failure(&mut self, pos: usize, expected: &'static str)
     -> RuleResult<()> {
        if pos > self.max_err_pos {
            self.max_err_pos = pos;
            self.expected.clear();
        }
        if pos == self.max_err_pos { self.expected.insert(expected); }
        Failed
    }
}
fn parse_identifier<'input>(input: &'input str, state: &mut ParseState,
                            pos: usize) -> RuleResult<&'input str> {
    {
        let start_pos = pos;
        {
            let seq_res =
                {
                    let mut repeat_pos = pos;
                    let mut repeat_value = vec!();
                    loop  {
                        let pos = repeat_pos;
                        let step_res =
                            {
                                let seq_res =
                                    if input.len() > pos {
                                        let (ch, next) =
                                            char_range_at(input, pos);
                                        match ch {
                                            'a' ...'z' | '-' | 'A' ...'Z' |
                                            '_' => Matched(next, ()),
                                            _ =>
                                            state.mark_failure(pos,
                                                               "[a-z-A-Z_]"),
                                        }
                                    } else {
                                        state.mark_failure(pos, "[a-z-A-Z_]")
                                    };
                                match seq_res {
                                    Matched(pos, _) => {
                                        {
                                            let assert_res =
                                                if input.len() > pos {
                                                    let (ch, next) =
                                                        char_range_at(input,
                                                                      pos);
                                                    match ch {
                                                        '-' =>
                                                        Matched(next, ()),
                                                        _ =>
                                                        state.mark_failure(pos,
                                                                           "[-]"),
                                                    }
                                                } else {
                                                    state.mark_failure(pos,
                                                                       "[-]")
                                                };
                                            match assert_res {
                                                Failed => Matched(pos, ()),
                                                Matched(..) => Failed,
                                            }
                                        }
                                    }
                                    Failed => Failed,
                                }
                            };
                        match step_res {
                            Matched(newpos, value) => {
                                repeat_pos = newpos;
                                repeat_value.push(value);
                            }
                            Failed => { break ; }
                        }
                    }
                    if repeat_value.len() >= 1usize {
                        Matched(repeat_pos, ())
                    } else { Failed }
                };
            match seq_res {
                Matched(pos, _) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { match_str })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_parser_type<'input>(input: &'input str, state: &mut ParseState,
                             pos: usize) -> RuleResult<&'input str> {
    parse_identifier(input, state, pos)
}
fn parse_parser_name<'input>(input: &'input str, state: &mut ParseState,
                             pos: usize) -> RuleResult<&'input str> {
    parse_identifier(input, state, pos)
}
fn parse_literal<'input>(input: &'input str, state: &mut ParseState,
                         pos: usize) -> RuleResult<&'input str> {
    parse_identifier(input, state, pos)
}
fn parse_part_parser<'input>(input: &'input str, state: &mut ParseState,
                             pos: usize) -> RuleResult<NodeType<'input>> {
    {
        let start_pos = pos;
        {
            let seq_res = slice_eq(input, state, pos, "%{");
            match seq_res {
                Matched(pos, _) => {
                    {
                        let seq_res = parse_parser_type(input, state, pos);
                        match seq_res {
                            Matched(pos, pt) => {
                                {
                                    let seq_res =
                                        slice_eq(input, state, pos, ":");
                                    match seq_res {
                                        Matched(pos, _) => {
                                            {
                                                let seq_res =
                                                    parse_parser_name(input,
                                                                      state,
                                                                      pos);
                                                match seq_res {
                                                    Matched(pos, pin) => {
                                                        {
                                                            let seq_res =
                                                                slice_eq(input,
                                                                         state,
                                                                         pos,
                                                                         "}");
                                                            match seq_res {
                                                                Matched(pos,
                                                                        _) =>
                                                                {
                                                                    {
                                                                        let match_str =
                                                                            &input[start_pos..pos];
                                                                        match {
                                                                                  if pt
                                                                                         ==
                                                                                         "SET"
                                                                                     {
                                                                                      let parser =
                                                                                          Box::new(SetParser::new(pin,
                                                                                                                  "0123456789"));
                                                                                      Ok(NodeType::Parser(parser))
                                                                                  } else {
                                                                                      Err("No parser found with this type")
                                                                                  }
                                                                              }
                                                                            {
                                                                            Ok(res)
                                                                            =>
                                                                            Matched(pos,
                                                                                    res),
                                                                            Err(expected)
                                                                            =>
                                                                            {
                                                                                state.mark_failure(pos,
                                                                                                   expected);
                                                                                Failed
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Failed =>
                                                                Failed,
                                                            }
                                                        }
                                                    }
                                                    Failed => Failed,
                                                }
                                            }
                                        }
                                        Failed => Failed,
                                    }
                                }
                            }
                            Failed => Failed,
                        }
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_part_literal<'input>(input: &'input str, state: &mut ParseState,
                              pos: usize) -> RuleResult<NodeType<'input>> {
    {
        let start_pos = pos;
        {
            let seq_res = parse_literal(input, state, pos);
            match seq_res {
                Matched(pos, lit) => {
                    {
                        let match_str = &input[start_pos..pos];
                        Matched(pos, { NodeType::Literal(lit) })
                    }
                }
                Failed => Failed,
            }
        }
    }
}
fn parse_pattern<'input>(input: &'input str, state: &mut ParseState,
                         pos: usize) -> RuleResult<CompiledPattern<'input>> {
    {
        let mut repeat_pos = pos;
        let mut repeat_value = vec!();
        loop  {
            let pos = repeat_pos;
            let step_res = parse_pattern_parts(input, state, pos);
            match step_res {
                Matched(newpos, value) => {
                    repeat_pos = newpos;
                    repeat_value.push(value);
                }
                Failed => { break ; }
            }
        }
        Matched(repeat_pos, repeat_value)
    }
}
fn parse_pattern_parts<'input>(input: &'input str, state: &mut ParseState,
                               pos: usize) -> RuleResult<NodeType<'input>> {
    {
        let choice_res = parse_part_parser(input, state, pos);
        match choice_res {
            Matched(pos, value) => Matched(pos, value),
            Failed => parse_part_literal(input, state, pos),
        }
    }
}
pub fn part_parser<'input>(input: &'input str)
 -> ParseResult<NodeType<'input>> {
    let mut state = ParseState::new();
    match parse_part_parser(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}
pub fn pattern<'input>(input: &'input str)
 -> ParseResult<CompiledPattern<'input>> {
    let mut state = ParseState::new();
    match parse_pattern(input, &mut state, 0) {
        Matched(pos, value) => { if pos == input.len() { return Ok(value) } }
        _ => { }
    }
    let (line, col) = pos_to_line(input, state.max_err_pos);
    Err(ParseError{line: line,
                   column: col,
                   offset: state.max_err_pos,
                   expected: state.expected,})
}