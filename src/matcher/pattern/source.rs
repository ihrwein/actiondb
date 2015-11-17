use super::Pattern;
use matcher::BuildError;

pub type BuildResult = Result<Pattern, BuildError>;

pub trait Source: Iterator<Item=BuildResult> {}
pub type PatternSource = Source<Item=BuildResult>;

impl<T: Iterator<Item=BuildResult>> Source for T {}
