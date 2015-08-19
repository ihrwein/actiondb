use std::collections::BTreeMap;
use std::borrow::Borrow;

use matcher::result::MatchResult;
use super::Error;

#[derive(Clone, Debug)]
pub struct TestMessage {
    message: String,
    values: BTreeMap<String, String>,
    tags: Option<Vec<String>>
}

impl TestMessage {
    pub fn new(message: String, values: BTreeMap<String, String>, tags: Option<Vec<String>>) -> TestMessage {
        TestMessage{
            message: message,
            values: values,
            tags: tags
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    pub fn tags(&self) -> Option<&[String]> {
        self.tags.as_ref().map(|x| x.borrow())
    }

    pub fn test_result(&self, result: &MatchResult) -> Result<(), Error> {
        try!(self.test_tags(result));
        try!(self.test_pairs(result.pairs()));
        if let Some(values) = result.pattern().values() {
            try!(self.test_additional_values(values));
        }
        Ok(())
    }

    fn test_additional_values(&self, values: &BTreeMap<String, String>) -> Result<(), Error> {
        for (key, value) in values {
            try!(self.test_value(key, value));
        }
        Ok(())
    }

    pub fn test_pairs(& self, pairs: &[(&str, &str)]) -> Result<(), Error> {
        for &(key, value) in pairs {
            try!(self.test_value(key, value));
        }
        Ok(())
    }

    fn test_value(&self, key: &str, value: &str) -> Result<(), Error> {
        let expected_value = self.values().get(key).map(|x| x.borrow());
        if let Some(exp) = expected_value {
            if exp != value {
                Err(Error::value_not_match(key, exp, value))
            } else {
                Ok(())
            }
        } else {
            Err(Error::key_not_found(key))
        }
    }

    fn test_tags(&self, result: &MatchResult) -> Result<(), Error> {
        if let Some(expected_tags) = self.tags() {
            if let Some(got_tags) = result.pattern().tags() {
                try!(self.test_expected_tags_can_be_found_in_got_tags(expected_tags, got_tags, result));
            } else {
                return Err(self.report_unexpected_tags_error(result));
            }
        }
        Ok(())
    }

    fn test_expected_tags_can_be_found_in_got_tags(&self, expected_tags: &[String], got_tags: &[String], result: &MatchResult) -> Result<(), Error> {
        for i in expected_tags {
            if !got_tags.contains(i) {
                return Err(self.report_unexpected_tags_error(result));
            }
        }
        Ok(())
    }

    fn report_unexpected_tags_error(&self, result: &MatchResult) -> Error {
        let expected = self.tags().map(|tags| { tags.to_vec() });
        let got = result.pattern().tags().map(|tags| { tags.to_vec() });
        Error::unexpected_tags(expected, got)
    }
}
