use matcher::Matcher;

pub trait MatcherFactory {
    type Matcher: Matcher;
    fn new_matcher() -> Self::Matcher;
}
