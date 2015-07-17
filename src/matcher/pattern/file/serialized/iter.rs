use super::SerializedPatternFile;
use matcher::pattern::Pattern;

use std::iter;
use std::vec;

impl iter::IntoIterator for SerializedPatternFile {
    type Item = Pattern;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.patterns.into_iter()
    }
}
