use uuid::Uuid;

use matcher::pattern::{Pattern, PatternSource};
use matcher::pattern::testmessage::{self, TestMessage};
use matcher::{Matcher, MatcherFactory};
use matcher::result::MatchResult;
use matcher::pattern::source::BuildResult;
pub use self::error::BuildError;

mod error;

pub trait FromPatternSource {
    fn from_source<F: MatcherFactory>(from: &mut PatternSource) -> Result<F::Matcher, BuildError> {
        let mut matcher = F::new_matcher();
        for pattern in from {
            try!(Self::check_pattern::<F::Matcher>(&mut matcher, pattern));
        }
        Ok(matcher)
    }

    fn from_source_ignore_errors<F: MatcherFactory>(from: &mut PatternSource) -> F::Matcher {
        let mut matcher = F::new_matcher();
        for pattern in from {
            let result = Self::check_pattern::<F::Matcher>(&mut matcher, pattern);
            if let Err(error) = result {
                error!("{}", error);
            }
        }
        matcher
    }

    fn check_pattern<M: Matcher>(matcher: &mut M, result: BuildResult) -> Result<(), BuildError> {
        let mut pattern = try!(result);
        let uuid = pattern.uuid().to_owned();
        let test_messages = Self::extract_test_messages(&mut pattern);
        matcher.add_pattern(pattern);
        debug!("validating pattern: {}", uuid.hyphenated().to_string());
        Self::check_test_messages(matcher, &test_messages, &uuid)
    }

    fn extract_test_messages(pattern: &mut Pattern) -> Vec<TestMessage> {
        let mut messages = Vec::new();

        while let Some(test_message) = pattern.pop_test_message() {
            messages.push(test_message);
        }
        messages
    }

    fn check_test_messages<M: Matcher>(matcher: &M,
                                       messages: &[TestMessage],
                                       uuid: &Uuid)
                                       -> Result<(), BuildError> {
        for msg in messages {
            let result = try!(matcher.parse(msg.message())
                                     .ok_or(testmessage::Error::test_message_does_not_match(uuid,
                                                                                            msg)));
            try!(Self::check_test_message(msg, &result, uuid));
        }
        Ok(())
    }

    fn check_test_message(message: &TestMessage,
                          result: &MatchResult,
                          expected_uuid: &Uuid)
                          -> Result<(), testmessage::Error> {
        if result.pattern().uuid() == expected_uuid {
            message.test_result(result)
        } else {
            Err(testmessage::Error::matched_to_other_pattern(expected_uuid,
                result.pattern().uuid(),
                message.message()))
        }
    }
}

impl<T> FromPatternSource for T where T: Matcher {
}
