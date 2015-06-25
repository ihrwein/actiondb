extern crate actiondb;
extern crate clap;

use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::{Arg, App, SubCommand};
use actiondb::grammar::parser;

const VERSION: &'static str = "0.1.0";
const AUTHOR: &'static str = "Tibor Benke <tibor.benke@balabit.com>";
const APPNAME: &'static str = "adbtool";

const PATTERN_FILE: &'static str = "pattern file";
const VALIDATE: &'static str = "validate";

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

fn main() {
    let matches = App::new(APPNAME)
                          .version(VERSION)
                          .author(AUTHOR)
                          .about("Tool for parsing unstructured data")
                          .subcommand(SubCommand::with_name("validate")
                                      .about("validates pattern file")
                                      .version(VERSION)
                                      .author(AUTHOR)
                                      .arg(Arg::with_name(PATTERN_FILE)
                                          .required(true)
                                          .index(1)
                                          .help("The pattern file to be validated")))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches(VALIDATE) {
        if !validate(matches.value_of(PATTERN_FILE).unwrap()) {
            std::process::exit(1);
        }
    } else {
        println!("{}", matches.usage.as_ref().unwrap());
    }
}
