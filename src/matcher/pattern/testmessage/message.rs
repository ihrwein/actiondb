use std::collections::BTreeMap;
use std::borrow::Borrow;

use matcher::result::MatchResult;
use super::Error;

#[derive(Clone, Debug)]
pub struct TestMessage {
    message: String,
    values: BTreeMap<String, String>
}

impl TestMessage {
    pub fn new(message: String, values: BTreeMap<String, String>) -> TestMessage {
        TestMessage{
            message: message,
            values: values
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    pub fn test_result(&self, result: &MatchResult) -> Result<(), Error> {
        try!(self.test_length(result));
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

    fn test_length(&self, result: &MatchResult) -> Result<(), Error> {
        let values_num = self.calc_values_number(result);
        if values_num != self.values().len() {
            Err(Error::invalid_length(self.values.len(), values_num))
        } else {
            Ok(())
        }
    }

    fn calc_values_number(&self, result: &MatchResult) -> usize {
        result.pairs().len() +
        result.pattern().values().map_or(0, |values| values.len())
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
}
