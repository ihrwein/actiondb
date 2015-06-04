pub use self::sortedvec::SortedVec;
pub use self::common_prefix::CommonPrefix;

use std::hash::{Hash, Hasher, SipHasher};

mod sortedvec;
// it shouldn't be public, but https://github.com/rust-lang/rust/issues/16264
pub mod common_prefix;

pub fn hash<T: Hash>(obj: T) -> u64
{
    let mut hasher = SipHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}
