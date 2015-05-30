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
}

impl CommonPrefix for str {
    fn common_prefix_len(&self, other: &Self) -> usize {
        let min_len = cmp::min(self.len(), other.len());
        let mut a_i = self.chars();
        let mut b_i = other.chars();

        for i in 0..min_len {
            let x = a_i.next();
            let y = b_i.next();

            if x != y {
                return i;
            }
        }
        return min_len;
    }
}

#[test]
fn given_a_string_when_longest_common_prefix_is_calulated_then_the_result_is_right() {
    let alpha = "alpha";
    let aleph = "aleph";
    let beta = "beta";

    assert_eq!(alpha.has_common_prefix(aleph).unwrap(), 2);
    assert_eq!(alpha.has_common_prefix(beta), None);
    assert_eq!(alpha.common_prefix_len(aleph), 2);
}
