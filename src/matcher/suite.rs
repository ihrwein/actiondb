use matcher::Matcher;
use matcher::MatcherFactory;
use parsers::ParserFactory;

pub trait MatcherSuite {
    type Matcher: Matcher;
    type ParserFactory: ParserFactory;
    type MatcherFactory: MatcherFactory<Matcher=Self::Matcher>;
}
