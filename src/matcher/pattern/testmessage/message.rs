use std::collections::BTreeMap;
use std::borrow::Borrow;

use matcher::result::MatchResult;
use super::Error;

#[derive(Clone, Debug)]
pub struct TestMessage {
    message: String,
    values: BTreeMap<String, String>,
    tags: Option<Vec<String>>,
}

impl TestMessage {
    pub fn new(message: String,
               values: BTreeMap<String, String>,
               tags: Option<Vec<String>>)
               -> TestMessage {
        TestMessage {
            message: message,
            values: values,
            tags: tags,
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
        self.test_values(result)
    }

    fn test_values(&self, result: &MatchResult) -> Result<(), Error> {
        let merged_values = TestMessage::merge_values(result);

        for (key, value) in self.values() {
            try!(TestMessage::test_value(key, value, &merged_values, result));
        }
        Ok(())
    }

    fn test_value(key: &str,
                  value: &str,
                  values: &BTreeMap<&str, &str>,
                  result: &MatchResult)
                  -> Result<(), Error> {
        if let Some(got_value) = values.get(key) {
            let got_value: &str = got_value;
            if value == got_value {
                Ok(())
            } else {
                Err(Error::value_not_match(result.pattern().uuid(), key, value, got_value))
            }
        } else {
            Err(Error::key_not_found(result.pattern().uuid(), key))
        }
    }

    fn merge_values<'a>(result: &'a MatchResult) -> BTreeMap<&'a str, &'a str> {
        let mut merged_values: BTreeMap<&str, &str> = BTreeMap::new();

        if let Some(values) = result.pattern().values() {
            for (key, value) in values {
                merged_values.insert(key.borrow(), value.borrow());
            }
        }
        for (key, value) in result.values() {
            merged_values.insert(key, value);
        }

        merged_values
    }

    fn test_tags(&self, result: &MatchResult) -> Result<(), Error> {
        if let Some(expected_tags) = self.tags() {
            if let Some(got_tags) = result.pattern().tags() {
                try!(self.test_expected_tags_can_be_found_in_got_tags(expected_tags,
                                                                      got_tags,
                                                                      result));
            } else {
                return Err(self.report_unexpected_tags_error(result));
            }
        }
        Ok(())
    }

    fn test_expected_tags_can_be_found_in_got_tags(&self,
                                                   expected_tags: &[String],
                                                   got_tags: &[String],
                                                   result: &MatchResult)
                                                   -> Result<(), Error> {
        for i in expected_tags {
            if !got_tags.contains(i) {
                return Err(self.report_unexpected_tags_error(result));
            }
        }
        Ok(())
    }

    fn report_unexpected_tags_error(&self, result: &MatchResult) -> Error {
        let expected = self.tags().map(|tags| tags.to_vec());
        let got = result.pattern().tags().map(|tags| tags.to_vec());
        Error::unexpected_tags(result.pattern().uuid(), expected, got)
    }
}
