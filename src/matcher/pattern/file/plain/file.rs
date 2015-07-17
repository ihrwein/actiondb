use std::fs;
use std::io;
use std::io::BufReader;

use super::Error;

pub struct PlainPatternFile {
    reader: io::BufReader<fs::File>
}

impl PlainPatternFile {
    pub fn open(path: &str) -> Result<PlainPatternFile, Error> {
        let file = try!(fs::File::open(path));
        let reader = BufReader::new(file);
        Ok(PlainPatternFile{reader: reader})
    }

    pub fn reader(self) -> io::BufReader<fs::File> {
        self.reader
    }
}
