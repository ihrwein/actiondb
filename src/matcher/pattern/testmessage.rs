use std::collections::BTreeMap;
use std::borrow::Borrow;

use serde;
use serde::de::Deserialize;

#[derive(Clone, Debug)]
pub struct TestMessage {
    message: String,
    values: BTreeMap<String, String>
}

impl TestMessage {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    pub fn test_pairs(&self, pairs: &[(&str, &str)]) -> Result<(), TestPairsError> {
        if pairs.len() != self.values().len() {
            Err(TestPairsError::InvalidLength{expected: self.values.len(), got: pairs.len()})
        } else  {
            self.test_pairs_values(pairs)
        }
    }

    pub fn test_pairs_values(& self, pairs: &[(&str, &str)]) -> Result<(), TestPairsError> {
        for &(key, value) in pairs {
            let expected_value = self.values().get(key).map(|x| x.borrow());
            if let Some(exp) = expected_value {
                if exp != value {
                    return Err(TestPairsError::value_not_match(key, exp, value));
                }
            } else {
                return Err(TestPairsError::key_not_found(key));
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum TestPairsError {
    InvalidLength{expected: usize, got: usize},
    ValueNotMatch{key: String, expected_value: String, got_value: String},
    KeyNotFound{key: String}
}

impl TestPairsError {
    pub fn value_not_match(key: &str, expected_value: &str, got_value: &str) -> TestPairsError {
        TestPairsError::ValueNotMatch{key: key.to_string(), expected_value: expected_value.to_string(), got_value: got_value.to_string()}
    }

    pub fn key_not_found(key: &str) -> TestPairsError {
        TestPairsError::KeyNotFound{key: key.to_string()}
    }
}

impl serde::Deserialize for TestMessage {
    fn deserialize<D>(deserializer: &mut D) -> Result<TestMessage, D::Error>
        where D: serde::de::Deserializer
    {
        deserializer.visit_named_map("TestMessage", TestMessageVisitor)
    }
}

enum Field {
    MESSAGE,
    VALUES,
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
                    "message" => Ok(Field::MESSAGE),
                    "values" => Ok(Field::VALUES),
                    _ => Err(serde::de::Error::syntax_error()),
                }
            }
        }

        deserializer.visit(FieldVisitor)
    }
}


struct TestMessageVisitor;

impl serde::de::Visitor for TestMessageVisitor {
    type Value = TestMessage;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<TestMessage, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut message = None;
        let mut values = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Field::MESSAGE) => { message = Some(try!(visitor.visit_value())); }
                Some(Field::VALUES) => { values = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let message_final = match message {
            Some(message) => message,
            None => try!(visitor.missing_field("message")),
        };

        let values_final = match values {
            Some(values) => values,
            None => try!(visitor.missing_field("values")),
        };

        try!(visitor.end());

        Ok(TestMessage{ message: message_final, values: values_final })
    }
}

#[cfg(test)]
mod test {
    use super::TestMessage;
    use serde::json;
    use std::borrow::Borrow;

    #[test]
    fn test_given_json_test_message_when_it_is_deserialized_then_we_get_the_right_instance() {
        let buffer = r#"
{
  "message": "lame-servers: info: unexpected RCODE (REFUSED) resolving 'ns1.example.org/AAAA/IN': 192.0.2.1#53",
  "values": {
    "dnsqry.query": "ns1.example.org",
    "dnsqry.type": "AAAA",
    "dnsqry.class": "IN",
    "dnsqry.client_ip": "192.0.2.1",
    "dnsqry.client_port": "53",
    "dnslame.reason": "unexpected RCODE (REFUSED)"
  }
}
"#;
        let result = json::from_str::<TestMessage>(buffer);
        println!("{:?}", result);
        let msg = result.ok().expect("Failed to deserialize a valid TestMessage from JSON");
        assert_eq!("lame-servers: info: unexpected RCODE (REFUSED) resolving 'ns1.example.org/AAAA/IN': 192.0.2.1#53", msg.message());
        assert_eq!(6, msg.values().len());
        assert_eq!(Some("AAAA"), msg.values().get("dnsqry.type").map(|x| x.borrow()));
    }
}
