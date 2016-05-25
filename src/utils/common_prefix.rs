use std::cmp;

pub trait CommonPrefix {
    fn has_common_prefix(&self, other: &Self) -> Option<usize> {
        let cpl = self.common_prefix_len(other);

        if cpl > 0 {
            Some(cpl)
        } else {
            None
        }
    }

    fn common_prefix_len(&self, other: &Self) -> usize;
    fn ltrunc(&self, len: usize) -> &Self;
    fn rtrunc(&self, len: usize) -> &Self;
}

impl CommonPrefix for str {
    fn common_prefix_len(&self, other: &Self) -> usize {
        let min_len = cmp::min(self.len(), other.len());
        let mut a_i = self.as_bytes().iter();
        let mut b_i = other.as_bytes().iter();
        let mut last_valid_character_boundary = 0;

        for i in 0..min_len {
            let x = a_i.next();
            let y = b_i.next();

            if self.is_char_boundary(i) {
                last_valid_character_boundary = i;
            }

            if x != y {
                return last_valid_character_boundary;
            }
        }
        min_len
    }

    fn ltrunc(&self, len: usize) -> &Self {
        &self[len..]
    }
    fn rtrunc(&self, len: usize) -> &Self {
        let new_len = self.len() - len;
        &self[..new_len]
    }
}

#[cfg(test)]
mod test {
    use utils::common_prefix::CommonPrefix;

    #[test]
    fn given_a_string_when_longest_common_prefix_is_calulated_then_the_result_is_right() {
        let alpha = "alpha";
        let aleph = "aleph";
        let beta = "beta";

        assert_eq!(alpha.has_common_prefix(aleph).unwrap(), 2);
        assert_eq!(alpha.has_common_prefix(beta), None);
        assert_eq!(alpha.common_prefix_len(aleph), 2);
    }

    #[test]
    fn test_given_a_string_when_truncated_by_left_then_the_result_is_the_expected() {
        assert_eq!("alpha".rtrunc(0), "alpha");
        assert_eq!("alpha".rtrunc(2), "alp");
    }

    #[test]
    fn test_given_a_string_when_truncated_by_right_then_the_result_is_the_expected() {
        assert_eq!("alpha".rtrunc(0), "alpha");
        assert_eq!("alpha".rtrunc(2), "alp");
    }

    #[test]
    fn test_given_a_string_with_multibyte_utf8_character_when_we_count_their_common_prefix_len_we_dont_split_the_multibyte_character() {
        assert_eq!("¡alpha".common_prefix_len("¡beta"), 2);
    }

    #[test]
    fn test_given_a_string_with_multibyte_utf8_character_when_there_is_an_other_multibyte_character_with_the_same_first_byte_then_we_dont_split_them() {
        assert_eq!("\u{00AE}alpha".common_prefix_len("\u{00BE}beta"), 0);
    }
}
