use uuid::Uuid;

use matcher::pattern::{Pattern, PatternSource};
use matcher::pattern::testmessage::{self, TestMessage};
use matcher::{Matcher, MatcherFactory};
use matcher::result::MatchResult;
use super::BuildError;

pub struct MatcherBuilder;

impl MatcherBuilder {
    pub fn from_source<F: MatcherFactory>(from: &mut PatternSource) -> Result<F::Matcher, BuildError> {
        let mut matcher = F::new_matcher();
        for pattern in from {
            let pattern = try!(pattern);
            try!(MatcherBuilder::check_pattern(pattern, &mut matcher));
        }
        Ok(matcher)
    }

    pub fn check_pattern(mut pattern: Pattern, matcher: &mut Matcher) -> Result<(), BuildError> {
        let uuid = pattern.uuid().clone();
        let test_messages = MatcherBuilder::extract_test_messages(&mut pattern);
        matcher.add_pattern(pattern);
        MatcherBuilder::check_test_messages(matcher, &test_messages, &uuid)
    }

    fn extract_test_messages(pattern: &mut Pattern) -> Vec<TestMessage> {
        let mut messages = Vec::new();

        while let Some(test_message) = pattern.pop_test_message() {
            messages.push(test_message);
        }
        messages
    }

    fn check_test_messages(matcher: &Matcher,
                           messages: &[TestMessage],
                           uuid: &Uuid)
                           -> Result<(), BuildError> {
        for msg in messages {
            let result = try!(matcher.parse(msg.message())
                                     .ok_or(testmessage::Error::test_message_does_not_match(uuid,
                                                                                            msg)));
            try!(MatcherBuilder::check_test_message(msg, &result, uuid));
        }
        Ok(())
    }

    fn check_test_message(message: &TestMessage,
                          result: &MatchResult,
                          expected_uuid: &Uuid)
                          -> Result<(), testmessage::Error> {
        if result.pattern().uuid() != expected_uuid {
            Err(testmessage::Error::matched_to_other_pattern(expected_uuid,
                                                             result.pattern().uuid(),
                                                             message.message()))
        } else {
            message.test_result(&result)
        }
    }
}
