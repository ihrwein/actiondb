extern crate actiondb;
extern crate clap;
#[macro_use]
extern crate log;

mod logger;
mod parse;

use clap::{Arg, App, SubCommand, ArgMatches};
use actiondb::matcher::PatternLoader;
use actiondb::matcher::suffix_array::SuffixArrayMatcherSuite;
use actiondb::matcher::MatcherSuite;
use log::LogLevelFilter;
use self::logger::StdoutLogger;

const AUTHOR: &'static str = "Tibor Benke <tibor.benke@balabit.com>";
const APPNAME: &'static str = "adbtool";
const DEBUG: &'static str = "debug";

const PATTERN_FILE: &'static str = "pattern file";
const VALIDATE: &'static str = "validate";
const PARSE: &'static str = "parse";
const IGNORE_ERRORS: &'static str = "ignore-errors";
const INPUT_FILE: &'static str = "input file";
const OUTPUT_FILE: &'static str = "output file";

fn build_command_line_argument_parser<'a, 'b>() -> App<'a, 'b> {
    let version = env!("CARGO_PKG_VERSION");
    App::new(APPNAME)
        .version(version)
        .author(AUTHOR)
        .about("Tool for parsing unstructured data")
        .arg(Arg::with_name(DEBUG)
                 .short("d")
                 .help("Enable debug messages"))
        .subcommand(SubCommand::with_name(VALIDATE)
                        .about("validates pattern file")
                        .version(version)
                        .author(AUTHOR)
                        .arg(Arg::with_name(PATTERN_FILE)
                                 .required(true)
                                 .index(1)
                                 .help("The pattern file to be validated"))
                        .arg(Arg::with_name(IGNORE_ERRORS)
                                 .short("i")
                                 .help("Don't stop at the first test message error")))
        .subcommand(SubCommand::with_name(PARSE)
                        .about("parses a file based on predefined patterns")
                        .version(version)
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
}

fn handle_validate<MS: MatcherSuite>(matches: &ArgMatches) {
    let pattern_file = matches.value_of(PATTERN_FILE).unwrap();

    let matcher = if matches.is_present(IGNORE_ERRORS) {
        PatternLoader::from_file_ignore_errors::<MS::MatcherFactory>(pattern_file)
    } else {
        PatternLoader::from_file::<MS::MatcherFactory>(pattern_file)
    };

    if let Err(e) = matcher {
        error!("{}", e);
        std::process::exit(1);
    }
}

fn handle_parse<MS: MatcherSuite>(matches: &ArgMatches) {
    let pattern_file = matches.value_of(PATTERN_FILE).unwrap();
    let input_file = matches.value_of(INPUT_FILE).unwrap();
    let output_file = matches.value_of(OUTPUT_FILE).unwrap();

    if let Err(e) = parse::parse::<MS>(pattern_file, input_file, output_file) {
        error!("{}", e);
        std::process::exit(1);
    }
}

fn setup_stdout_logger(log_level: LogLevelFilter) {
    let _ = log::set_logger(|max_log_level| {
        max_log_level.set(log_level);
        Box::new(StdoutLogger)
    });
}

fn choose_log_level<'a>(matches: &ArgMatches<'a>) -> LogLevelFilter {
    if matches.is_present(DEBUG) {
        LogLevelFilter::Debug
    } else {
        LogLevelFilter::Info
    }
}

fn process_command_line_args<'a, MS: MatcherSuite>(matches: ArgMatches<'a>) {
    if let Some(matches) = matches.subcommand_matches(VALIDATE) {
        handle_validate::<MS>(&matches);
    } else if let Some(matches) = matches.subcommand_matches(PARSE) {
        handle_parse::<MS>(&matches);
    } else {
        error!("{}", matches.usage.as_ref().unwrap());
    }
}

fn main() {
    let matches = build_command_line_argument_parser().get_matches();
    let log_level = choose_log_level(&matches);
    setup_stdout_logger(log_level);
    process_command_line_args::<SuffixArrayMatcherSuite>(matches);
}
