# ex_merge_sort_by_key

Rust [external merge sort](https://en.wikipedia.org/wiki/External_sorting#External_merge_sort) library using [sort_by_cached_key](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_cached_key).

[![GitHub](https://img.shields.io/badge/github-winebarrel/ex__merge__sort__by__key-safegreen?logo=github)](https://github.com/winebarrel/ex_merge_sort_by_key)
[![Build Status](https://github.com/winebarrel/ex_merge_sort_by_key/workflows/CI/badge.svg)](https://github.com/winebarrel/ex_merge_sort_by_key/actions)
[![crates.io](https://img.shields.io/crates/v/ex_merge_sort_by_key.svg)](https://crates.io/crates/ex_merge_sort_by_key)
[![docs.rs](https://docs.rs/ex_merge_sort_by_key/badge.svg)](https://docs.rs/ex_merge_sort_by_key)

## Usage

```toml
[dependencies]
ex_merge_sort_by_key = "0.1"
```

```rust
use std::fs::File;
use std::io;

fn main() {
    let f = File::open("README.md").unwrap();
    let capacity = 1024;
    ex_merge_sort_by_key::sort_by_key(f, io::stdout(), capacity, |line| line.len()).unwrap();
}
```
