use matcher::pattern::{Pattern, PatternSource};
use matcher::pattern::testmessage::{TestMessage, TestPairsError};
use matcher::Matcher;
use super::BuildError;

pub struct Builder;

impl Builder {
    pub fn drain_into(from: &mut PatternSource, matcher: &mut Matcher) -> Result<(), BuildError>{
        for pattern in from {
            let mut pattern = try!(pattern);
            let test_messages = Builder::extract_test_messages(&mut pattern);
            matcher.add_pattern(pattern);
            try!(Builder::check_test_messages(matcher, &test_messages));
        }
        Ok(())
    }

    fn extract_test_messages(pattern: &mut Pattern) -> Vec<TestMessage> {
        let mut messages = Vec::new();

        while let Some(test_message) = pattern.pop_test_message() {
            messages.push(test_message);
        }
        messages
    }

    fn check_test_messages(matcher: &Matcher, messages: &[TestMessage]) -> Result<(), BuildError> {
        for msg in messages {
            let result = try!(matcher.parse(msg.message()).ok_or(TestPairsError::TestMessageDoesntMatch));
            try!(msg.test_pairs(result.pairs()));
        }
        Ok(())
    }
}
