use super::Pattern;
use super::testmessage::TestMessage;

use serde::de::Deserialize;
use grammar::pattern_parser;
use serde;
use uuid::Uuid;

impl serde::Deserialize for Pattern {
    fn deserialize<D>(deserializer: &mut D) -> Result<Pattern, D::Error>
        where D: serde::de::Deserializer
    {
        deserializer.visit_named_map("Pattern", PatternVisitor)
    }
}

enum Field {
    NAME,
    UUID,
    PATTERN,
    TESTMESSAGES,
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
                    "name" => Ok(Field::NAME),
                    "uuid" => Ok(Field::UUID),
                    "pattern" => Ok(Field::PATTERN),
                    "test_messages" => Ok(Field::TESTMESSAGES),
                    _ => Err(serde::de::Error::syntax_error()),
                }
            }
        }

        deserializer.visit(FieldVisitor)
    }
}


struct PatternVisitor;

impl serde::de::Visitor for PatternVisitor {
    type Value = Pattern;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Pattern, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut name = None;
        let mut uuid = None;
        let mut pattern: Option<String> = None;
        let mut test_messages: Option<Vec<TestMessage>> = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Field::NAME) => { name = Some(try!(visitor.visit_value())); }
                Some(Field::UUID) => {
                    let value: String = try!(visitor.visit_value());
                    uuid = match Uuid::parse_str(&value) {
                        Ok(v) => Some(v),
                        Err(_) => try!(Err(serde::de::Error::missing_field_error("uuid")))
                    }
                }
                Some(Field::PATTERN) => { pattern = Some(try!(visitor.visit_value())); }
                Some(Field::TESTMESSAGES) => { test_messages = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let name = match name {
            Some(name) => name,
            None => try!(visitor.missing_field("name")),
        };

        let pattern = match pattern {
            Some(pattern) => pattern,
            None => try!(visitor.missing_field("pattern")),
        };

        let uuid_final = match uuid {
            Some(uuid) => uuid,
            None => return Err(serde::de::Error::missing_field_error("uuid"))
        };

        try!(visitor.end());

        let pattern_final = pattern_parser::pattern(&pattern).unwrap();

        Ok(Pattern{ name: name, uuid: uuid_final, pattern: pattern_final, test_messages: test_messages })
    }
}
