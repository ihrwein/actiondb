pub use self::sortedvec::SortedVec;
pub use self::common_prefix::CommonPrefix;

mod sortedvec;
// it shouldn't be public, but https://github.com/rust-lang/rust/issues/16264
pub mod common_prefix;

pub fn flatten_vec<T>(vectors: Vec<Vec<T>>) -> Vec<T> {
    let mut flattened_vec = Vec::new();
    for vector in vectors.into_iter() {
        for i in vector.into_iter() {
            flattened_vec.push(i);
        }
    }
    flattened_vec
}
