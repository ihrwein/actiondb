use matcher::trie::ParserTrie;
use matcher::pattern::{Pattern, PatternSource};
use matcher::errors::FromJsonError;
use matcher::pattern::testmessage::TestMessage;

use super::BuildError;

pub struct Builder;

impl Builder {
    pub fn drain_into(from: &mut PatternSource, to: &mut ParserTrie) -> Result<(), BuildError>{
        for mut pattern in from {
            let test_messages = Builder::extract_test_messages_from_pattern(&mut pattern);
            to.insert(pattern);
            try!(Builder::check_test_messages_on_trie(&to, &test_messages));
        }

        Ok(())
    }

    fn extract_test_messages_from_pattern(pattern: &mut Pattern) -> Vec<TestMessage> {
        let mut messages = Vec::new();

        while let Some(test_message) = pattern.pop_test_message() {
            messages.push(test_message);
        }
        messages
    }

    fn check_test_messages_on_trie(trie: &ParserTrie, messages: &[TestMessage]) -> Result<(), FromJsonError> {
        for msg in messages {
            if let Some(result) = trie.parse(msg.message()) {
                try!(msg.test_pairs(result.pairs()));
            } else {
                return Err(FromJsonError::TestMessageDoesntMatch);
            }
        }
        Ok(())
    }
}
