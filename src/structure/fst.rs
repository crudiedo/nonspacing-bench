use fst::raw::Fst;
use fst::Set;
use once_cell::sync::Lazy;


pub static FST: Lazy<Set<&[u8]>> = Lazy::new(|| {
    let chars: &[u8] = include_bytes!("../../dicts/fst/chars.fst");

    Set::new(chars).unwrap()
});


pub fn fst_filter() -> fn(char) -> bool {
    |c| -> bool { FST.contains(c.to_string()) }
}