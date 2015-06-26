use std::fs::File;
use std::io::{BufReader, BufRead, Error, BufWriter, Write};
use actiondb::grammar::parser;
use actiondb::matcher::trie::ParserTrie;

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

pub fn parse(pattern_file_path: &str, input_file_path: &str, output_file_path: &str) -> Result<(), Error> {
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
