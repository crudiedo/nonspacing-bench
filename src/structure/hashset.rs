use std::collections::HashSet;
use once_cell::sync::Lazy;
use crate::structure::bytes::get_bytes;

pub static HASHSET: Lazy<HashSet<u32>> = Lazy::new(|| {
    HashSet::from_iter(get_bytes())
});


pub fn hashset_filter() -> fn(char) -> bool {
    |c| -> bool {HASHSET.contains(&(c as u32))}
}