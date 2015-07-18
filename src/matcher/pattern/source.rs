use super::Pattern;

pub trait Source: Iterator<Item=Pattern> {}
pub type PatternSource = Source<Item=Pattern>;

impl<T: Iterator<Item=Pattern>> Source for T {}
