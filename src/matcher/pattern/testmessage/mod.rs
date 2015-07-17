use std::collections::BTreeMap;
use std::borrow::Borrow;

mod deser;

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
            Err(TestPairsError::invalid_length(self.values.len(), pairs.len()))
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
    pub fn invalid_length(expected: usize, got: usize) -> TestPairsError {
        TestPairsError::InvalidLength{expected: expected, got: got}
    }

    pub fn value_not_match(key: &str, expected_value: &str, got_value: &str) -> TestPairsError {
        TestPairsError::ValueNotMatch{key: key.to_string(), expected_value: expected_value.to_string(), got_value: got_value.to_string()}
    }

    pub fn key_not_found(key: &str) -> TestPairsError {
        TestPairsError::KeyNotFound{key: key.to_string()}
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
