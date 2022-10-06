use once_cell::sync::Lazy;
use roaring::RoaringBitmap;
use crate::structure::bytes::{get_bytes};


pub static ROARING: Lazy<RoaringBitmap> = Lazy::new(|| {
    RoaringBitmap::from_sorted_iter(get_bytes())
        .unwrap()
});

pub fn roaring_filter() -> fn(char) -> bool {
    |c| -> bool {ROARING.contains(c as u32)}
}