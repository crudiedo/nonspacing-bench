use std::collections::BTreeSet;
use once_cell::sync::Lazy;
use crate::structure::bytes::{get_bytes};

pub static BTREE_SET: Lazy<BTreeSet<u32>> = Lazy::new(|| {
    BTreeSet::from_iter(get_bytes())
});


pub fn btree_filter() -> fn(char) -> bool {
    |c| -> bool {BTREE_SET.contains(&(c as u32))}
}