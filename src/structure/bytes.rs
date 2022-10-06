use std::iter::Map;
use std::slice::ChunksExact;

pub fn get_bytes<'a>() -> Vec<u32> {
    let bytes = include_bytes!("../../dicts/bin/chars.bin");

    bytes
        .chunks_exact(4)
        .map(|chunk| {
            u32::from_ne_bytes(chunk.try_into().unwrap())
        }).collect::<Vec<u32>>()
}