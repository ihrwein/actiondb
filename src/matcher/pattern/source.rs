use super::Pattern;

pub trait PatternSource: Iterator<Item=Pattern> {}

impl<T: Iterator<Item=Pattern>> PatternSource for T {}
