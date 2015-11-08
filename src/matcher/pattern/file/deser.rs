use serde;

use super::PatternFile;
use matcher::pattern::Pattern;

impl serde::Deserialize for PatternFile {
    fn deserialize<D>(deserializer: &mut D) -> Result<PatternFile, D::Error>
        where D: serde::de::Deserializer
    {
        deserializer.visit_struct("File", &[], FileVisitor)
    }
}

enum Field {
    PATTERNS,
}

impl serde::Deserialize for Field {
    fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
        where D: serde::de::Deserializer
    {
        struct FieldVisitor;

        impl serde::de::Visitor for FieldVisitor {
            type Value = Field;

            fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                where E: serde::de::Error
            {
                match value {
                    "patterns" => Ok(Field::PATTERNS),
                    _ => Err(serde::de::Error::syntax(&format!("Unexpected field: {}", value))),
                }
            }
        }

        deserializer.visit(FieldVisitor)
    }
}

struct FileVisitor;

impl serde::de::Visitor for FileVisitor {
    type Value = PatternFile;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<PatternFile, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut patterns: Option<Vec<Pattern>> = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Field::PATTERNS) => {
                    patterns = Some(try!(visitor.visit_value()));
                }
                None => {
                    break;
                }
            }
        }

        let patterns_final = match patterns {
            Some(patterns) => patterns,
            None => try!(visitor.missing_field("patterns")),
        };

        try!(visitor.end());

        Ok(PatternFile { patterns: patterns_final })
    }
}
