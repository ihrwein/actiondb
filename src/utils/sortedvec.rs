use std::cmp::Ordering;

#[derive(Debug)]
pub struct SortedVec<T> {
    array: Vec<T>
}

impl <T: Ord> SortedVec<T> {
    pub fn new() -> SortedVec<T> {
        SortedVec{ array: vec!() }
    }

    pub fn push(&mut self, value: T) {
        self.array.push(value);
        self.insertion_sort();
    }

    pub fn find_pos(&self, value: &T) -> Option<usize> {
        self.binary_search(value)
    }

    pub fn find(&self, value: &T) -> Option<&T> {
        if let Some(index) = self.binary_search(value) {
            self.array.get(index)
        } else {
            None
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.array.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.array.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.array.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize> where F: FnMut(&T) -> Ordering {
        self.array.binary_search_by(f)
    }

    fn insertion_sort(&mut self) {
        for i in 1..(self.array.len() - 1) {
            let mut j = i;

            while j > 0 && self.array.get(j - 1).unwrap() > self.array.get(j).unwrap() {
                self.array.swap(j, j - 1);
                j = j - 1;
            }
        }
    }

    fn binary_search<'a>(&self, needle: &T) -> Option<usize> {
        let mut low  = 0;
        let mut high = self.array.len();

        while low < high {
            let mid = (low + high) / 2;

            match self.array[mid].cmp(needle) {
                Ordering::Less => low = mid +1,
                Ordering::Greater => high = mid,
                Ordering::Equal => return Some(mid)
            };
        }

        None
    }
}

#[cfg(test)]
mod test {
    use utils::SortedVec;
    use std::cmp::Ordering;

    #[test]
    fn test_given_sorted_vector_when_values_are_pushed_they_be_get() {
        let mut sv = SortedVec::new();
        sv.push("alpha".to_owned());
        assert_eq!(sv.get(0).unwrap(), "alpha");
    }

    #[test]
    fn test_given_sorted_vector_when_values_are_pushed_they_get_sorted() {
        let mut sv = SortedVec::new();

        sv.push("epsilon".to_owned());
        sv.push("beta".to_owned());
        sv.push("alpha".to_owned());
        sv.push("delta".to_owned());
        sv.push("zeta".to_owned());

        assert_eq!(sv.get(0).unwrap(), "alpha");
        assert_eq!(sv.get(1).unwrap(), "beta");
        assert_eq!(sv.get(2).unwrap(), "delta");
        assert_eq!(sv.get(3).unwrap(), "epsilon");
        assert_eq!(sv.get(4).unwrap(), "zeta");
    }

    #[test]
    fn test_given_sorted_vector_when_values_are_searched_they_can_be_found() {
        let mut sv = SortedVec::new();

        sv.push("epsilon".to_owned());
        sv.push("beta".to_owned());
        sv.push("alpha".to_owned());
        sv.push("delta".to_owned());
        sv.push("zeta".to_owned());

        assert_eq!(sv.find_pos(&"beta".to_owned()).unwrap(), 1);
        assert_eq!(sv.find_pos(&"zeta".to_owned()).unwrap(), 4);
    }

    #[test]
    fn test_given_sorted_vector_when_length_is_queried_it_is_ok() {
        let mut sv = SortedVec::new();

        sv.push("epsilon".to_owned());
        sv.push("beta".to_owned());

        assert_eq!(sv.len(), 2);
    }

    #[test]
    fn test_given_sorted_vector_when_values_are_found_then_their_references_are_returned() {
        let mut sv = SortedVec::new();

        sv.push("epsilon");
        sv.push("beta");
        sv.push("alpha");
        sv.push("delta");
        sv.push("zeta");

        assert_eq!(sv.find(&"beta"), Some(&"beta"));
        assert_eq!(sv.find(&"zeta"), Some(&"zeta"));
        // Half-Life 3 hasn't been released yet, how could we find it?
        assert_eq!(sv.find(&"<3 HL3"), None);
    }

    #[test]
    fn test_given_sorted_vector_when_values_are_searched_by_custom_cmp_func_they_can_be_found() {
        let mut sv = SortedVec::new();

        sv.push("epsilon");
        sv.push("beta");
        sv.push("alpha");
        sv.push("delta");
        sv.push("zeta");

        assert_eq!(sv.binary_search_by(|x: &&str| {x.cmp(&"iota")}), Err(4));
        assert_eq!(sv.binary_search_by(|x: &&str| {x.cmp(&"beta")}), Ok(1));
    }
}
