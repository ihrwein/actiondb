use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind, BufWriter, Write};
use actiondb::Matcher;
use actiondb::matcher;

pub fn parse(pattern_file_path: &str, input_file_path: &str, output_file_path: &str) -> Result<(), Error> {
    match matcher::Factory::from_plain_file(pattern_file_path) {
        Ok(matcher) => {
            let input_file = try!(File::open(input_file_path));
            let mut output_file= try!(File::create(output_file_path));
            parse_file(&input_file, &mut output_file, &matcher);
            Ok(())
        },
        Err(err) => {
            Err(Error::new(ErrorKind::Other, format!("Failed to parse a pattern in the input file: {:?}", err)))
        }
    }
}

fn parse_file(input_file: &File, output_file: &mut File, matcher: &Box<Matcher>) {
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    for line in reader.lines() {
        if let Ok(l) = line {
            let parse_result = matcher.parse(&l);
            let _ = write!(&mut writer, "{:?}\n", parse_result);
        }
    }
}
