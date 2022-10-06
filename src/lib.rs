use std::collections::BTreeSet;
use crate::structure::btree_set::{btree_filter};
use crate::structure::hashset::{hashset_filter};
use crate::structure::naive::{naive, naive_filter};
use crate::structure::roaring_bitmap::{roaring_filter};

pub mod structure;


pub fn run() {
    let naive_filtered = load_and_filter(naive_filter());
    let btree_filtered = load_and_filter(btree_filter());
    let roaring_filtered = load_and_filter(roaring_filter());
    let hashset_filtered = load_and_filter(hashset_filter());

    println!("Naive: {} chars filtered", naive_filtered);
    println!("BTreeSet: {} chars filtered", btree_filtered);
    println!("RoaringBitmap: {} chars filtered", roaring_filtered);
    println!("HashSet: {} chars filtered", hashset_filtered);

    assert_eq!(
        naive_filtered,
        btree_filtered,
    );
    assert_eq!(
        naive_filtered,
        roaring_filtered,
    );
    assert_eq!(
        naive_filtered,
        hashset_filtered,
    );
}

/// Loads the provided file and filters its chars one by one
/// Returns a number of "filtered" chars
pub fn load_and_filter<F: Fn(char) -> bool>(f: F) -> u32 {
    let mut filtered = 0;
    let content = include_str!("../samples/arabic.txt");

    for char in content.chars() {
        if f(char) {
            filtered += 1;
        }
    }


    filtered
}