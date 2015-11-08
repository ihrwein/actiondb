pub trait HasLengthConstraint {
    fn min_length(&self) -> Option<usize>;
    fn set_min_length(&mut self, length: Option<usize>);
    fn max_length(&self) -> Option<usize>;
    fn set_max_length(&mut self, length: Option<usize>);

    fn is_match_length_ok(&self, match_length: usize) -> bool {
        match_length > 0 && self.is_min_length_ok(match_length) &&
        self.is_max_length_ok(match_length)
    }

    fn is_min_length_ok(&self, match_length: usize) -> bool {
        match self.min_length() {
            Some(x) => match_length >= x,
            None => true,
        }
    }

    fn is_max_length_ok(&self, match_length: usize) -> bool {
        match self.max_length() {
            Some(x) => match_length <= x,
            None => true,
        }
    }
}
