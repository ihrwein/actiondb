extern crate actiondb;
extern crate clap;

use std::fs::File;
use std::io::{BufReader, BufRead, Error, BufWriter, Write};
use clap::{Arg, App, SubCommand};
use actiondb::grammar::parser;
use actiondb::matcher::trie::ParserTrie;

const VERSION: &'static str = "0.1.0";
const AUTHOR: &'static str = "Tibor Benke <tibor.benke@balabit.com>";
const APPNAME: &'static str = "adbtool";

const PATTERN_FILE: &'static str = "pattern file";
const VALIDATE: &'static str = "validate";
const PARSE: &'static str = "parse";
const INPUT_FILE: &'static str = "input file";
const OUTPUT_FILE: &'static str = "output file";

fn validate_file(file: &File) -> bool {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(l) = line {
            if let Err(err) = parser::pattern(&l) {
                println!("{:?}", err);
                return false;
            }
        }
    }

    true
}

fn validate(filename: &str) -> bool {
    match File::open(filename) {
        Ok(file) => {
            validate_file(&file)
        },
        Err(e) => {
            println!("{}", e);
            false
        }
    }
}

fn build_trie_from_pattern_file(file: &File) -> Result<ParserTrie, parser::ParseError> {
    let mut trie = ParserTrie::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(l) = line {
            let compiled_pattern = parser::pattern(&l);

            if compiled_pattern.is_ok() {
                trie.insert(compiled_pattern.ok().unwrap());
            } else {
                return Err(compiled_pattern.err().unwrap());
            }
        }
    }

    Ok(trie)
}

fn parse(pattern_file_path: &str, input_file_path: &str, output_file_path: &str) -> Result<(), Error> {
    let pattern_file = try!(File::open(pattern_file_path));
    let input_file = try!(File::open(input_file_path));
    let mut output_file= try!(File::create(output_file_path));

    let build_result = build_trie_from_pattern_file(&pattern_file);

    match build_result {
        Ok(trie) => {
            parse_file(&input_file, &mut output_file, &trie);
        },
        Err(err) => {
            println!("Failed to parse a pattern in the input file: {:?}", err);
        }
    }

    Ok(())
}

fn parse_file(input_file: &File, output_file: &mut File, trie: &ParserTrie) {
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    for line in reader.lines() {
        if let Ok(l) = line {
            let parse_result = trie.parse(&l);
            let _ = write!(&mut writer, "{:?}\n", parse_result);
        }
    }
}

fn main() {
    let matches = App::new(APPNAME)
                          .version(VERSION)
                          .author(AUTHOR)
                          .about("Tool for parsing unstructured data")
                          .subcommand(SubCommand::with_name(VALIDATE)
                                      .about("validates pattern file")
                                      .version(VERSION)
                                      .author(AUTHOR)
                                      .arg(Arg::with_name(PATTERN_FILE)
                                          .required(true)
                                          .index(1)
                                          .help("The pattern file to be validated")))
                          .subcommand(SubCommand::with_name(PARSE)
                                      .about("parses a file based on predefined patterns")
                                      .version(VERSION)
                                      .author(AUTHOR)
                                      .arg(Arg::with_name(PATTERN_FILE)
                                          .required(true)
                                          .index(1)
                                          .help("The pattern file which contains predefined patterns"))
                                      .arg(Arg::with_name(INPUT_FILE)
                                          .required(true)
                                          .index(2)
                                          .help("The input file to be parsed"))
                                      .arg(Arg::with_name(OUTPUT_FILE)
                                          .required(true)
                                          .index(3)
                                          .help("The output file where the results are written")))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches(VALIDATE) {
        if !validate(matches.value_of(PATTERN_FILE).unwrap()) {
            std::process::exit(1);
        }
    } else if let Some(matches) = matches.subcommand_matches(PARSE) {
        let pattern_file = matches.value_of(PATTERN_FILE).unwrap();
        let input_file = matches.value_of(INPUT_FILE).unwrap();
        let output_file = matches.value_of(OUTPUT_FILE).unwrap();

        if let Err(e) = parse(pattern_file, input_file, output_file) {
            println!("{}", e);
            std::process::exit(1);
        }
    } else {
        println!("{}", matches.usage.as_ref().unwrap());
    }
}
