use parsers::ParserBase;

#[derive(Clone, Hash, Debug)]
pub struct LengthCheckedParserBase {
    base: ParserBase,
    min_length: Option<usize>,
    max_length: Option<usize>,
}

impl LengthCheckedParserBase {
    pub fn with_name(name: String) -> LengthCheckedParserBase {
        LengthCheckedParserBase {
            base: ParserBase::with_name(name),
            min_length: None,
            max_length: None,
        }
    }

    pub fn new() -> LengthCheckedParserBase {
        LengthCheckedParserBase {
            base: ParserBase::new(),
            min_length: None,
            max_length: None,
        }
    }

    pub fn set_min_length(&mut self, length: usize) {
        self.min_length = Some(length);
    }

    pub fn set_max_length(&mut self, length: usize) {
        self.max_length = Some(length);
    }

    pub fn is_match_length_ok(&self, match_length: usize) -> bool {
        match_length > 0 && self.is_min_length_ok(match_length) &&
        self.is_max_length_ok(match_length)
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.base.set_name(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.base.name()
    }

    fn is_min_length_ok(&self, match_length: usize) -> bool {
        match self.min_length {
            Some(x) => match_length >= x,
            None => true,
        }
    }

    fn is_max_length_ok(&self, match_length: usize) -> bool {
        match self.max_length {
            Some(x) => match_length <= x,
            None => true,
        }
    }
}

#[cfg(test)]
mod test {
    use parsers::HasLengthConstraint;

    struct DummyImpl {
        min_length: Option<usize>,
        max_length: Option<usize>,
    }

    impl DummyImpl {
        fn new() -> DummyImpl {
            DummyImpl {
                min_length: None,
                max_length: None,
            }
        }
    }

    impl HasLengthConstraint for DummyImpl {
        fn min_length(&self) -> Option<usize> {
            self.min_length
        }
        fn set_min_length(&mut self, length: Option<usize>) {
            self.min_length = length;
        }
        fn max_length(&self) -> Option<usize> {
            self.max_length
        }
        fn set_max_length(&mut self, length: Option<usize>) {
            self.max_length = length;
        }
    }

    #[test]
    fn test_given_parser_when_the_match_length_is_not_constrained_then_the_match_length_is_ok_in_every_case
        () {
        let base = DummyImpl::new();
        assert_eq!(base.is_match_length_ok(42), true);
        assert_eq!(base.is_match_length_ok(1), true);
    }

    #[test]
    fn test_given_parser_when_the_minimum_match_length_is_set_then_the_shorter_matches_are_discarded() {
        let mut base = DummyImpl::new();
        base.set_min_length(Some(10));
        assert_eq!(base.is_match_length_ok(42), true);
        assert_eq!(base.is_match_length_ok(1), false);
        assert_eq!(base.is_match_length_ok(9), false);
        assert_eq!(base.is_match_length_ok(10), true);
    }

    #[test]
    fn test_given_parser_when_the_maximum_match_length_is_set_then_the_longer_matches_are_discarded
        () {
        let mut base = DummyImpl::new();
        base.set_max_length(Some(10));
        assert_eq!(base.is_match_length_ok(42), false);
        assert_eq!(base.is_match_length_ok(1), true);
        assert_eq!(base.is_match_length_ok(9), true);
        assert_eq!(base.is_match_length_ok(10), true);
    }
}
