# Nonspacing Bench

This repo is a sample app that's benchmarking different data structures to filter the nonspacing marks.

It's the result of that [charabia/issues/137](https://github.com/meilisearch/charabia/issues/137) issue. 

## Running benchmarks

Run ``cargo bench`` and that's it.  

You can also run `cargo run` to ensure that all the data structures filter the same number of chars.

## Results on my machine
```bash
Naive                   time:   [22.296 ms 22.378 ms 22.461 ms]  

BTreeSet                time:   [14.278 ms 14.335 ms 14.394 ms] 

RoaringBitmap           time:   [15.217 ms 15.274 ms 15.333 ms] 

HashSet                 time:   [9.5395 ms 9.5683 ms 9.5972 ms]   
```

It looks like HashSet is the fastest one, even though it takes more memory. 

## Memory Usage

I'm still not sure how to properly get the dynamic variable size in Rust, so I captured some samples with jemalloc and put them in `jemalloc_stats`.

## Text sample 

The repo is using an Arabic text sample from that repo [AliOsm/arabic-text-diacritization](https://github.com/AliOsm/arabic-text-diacritization)

## Nonspacing Marks 

The nonspacing marks are finite set (1281 chars) of unicode chars show there [https://www.compart.com/en/unicode/category/Mn](https://www.compart.com/en/unicode/category/Mn)

The repo stores them as bin and txt files inside dicts. 

The code uses bin representation only.

