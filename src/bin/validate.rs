use std::fs::File;
use std::io::{BufReader, BufRead};
use actiondb::grammar::parser;

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

pub fn validate(filename: &str) -> bool {
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
