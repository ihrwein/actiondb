pub use self::sortedvec::SortedVec;
pub use self::common_prefix::CommonPrefix;

mod sortedvec;
// it shouldn't be public, but https://github.com/rust-lang/rust/issues/16264
pub mod common_prefix;
