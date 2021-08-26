//! Rust [external merge sort](https://en.wikipedia.org/wiki/External_sorting#External_merge_sort) library using [sort_by_cached_key](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_cached_key).
//!
//! [![GitHub](https://img.shields.io/badge/github-winebarrel/ex__merge__sort__by__key-safegreen?logo=github)](https://github.com/winebarrel/ex_merge_sort_by_key)
//! [![Build Status](https://github.com/winebarrel/ex_merge_sort_by_key/workflows/CI/badge.svg)](https://github.com/winebarrel/ex_merge_sort_by_key/actions)
//! [![crates.io](https://img.shields.io/crates/v/ex_merge_sort_by_key.svg)](https://crates.io/crates/ex_merge_sort_by_key)
//! [![docs.rs](https://docs.rs/ex_merge_sort_by_key/badge.svg)](https://docs.rs/ex_merge_sort_by_key)
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! ex_merge_sort_by_key = "0.1"
//! ```
//!
//! ```rust
//! use std::fs::File;
//! use std::io;
//!
//! fn main() {
//!     let f = File::open("README.md").unwrap();
//!     let capacity = 1024;
//!     ex_merge_sort_by_key::sort_by_key(f, io::stdout(), capacity, |line| line.len()).unwrap();
//! }
//! ```

#[cfg(test)]
mod tests;

mod chunk;
mod file_utils;
mod slice_utils;

use chunk::Chunk;
use file_utils::RoughCount;
use io::prelude::BufRead;
use std::fs;
use std::io;
use std::io::Write;

pub fn sort_by_key<T, F, K>(fin: fs::File, fout: T, cap: u64, key: F) -> io::Result<()>
where
    T: io::Write,
    F: Fn(&String) -> K,
    K: Ord,
{
    sort_by_key_with_order(fin, fout, cap, false, key)
}

pub fn reverse_sort_by_key<T, F, K>(fin: fs::File, fout: T, cap: u64, key: F) -> io::Result<()>
where
    T: io::Write,
    F: Fn(&String) -> K,
    K: Ord,
{
    sort_by_key_with_order(fin, fout, cap, true, key)
}

pub fn sort_by_key_with_order<T, F, K>(
    fin: fs::File,
    fout: T,
    cap: u64,
    rev: bool,
    key: F,
) -> io::Result<()>
where
    T: io::Write,
    F: Fn(&String) -> K,
    K: Ord,
{
    let chunk = Chunk::new(fin, cap)?;
    let sorted = sort_chunk(chunk, rev, &key)?;
    file_utils::copy(&sorted.file, fout)
}

fn sort_chunk<F, K>(chunk: Chunk, rev: bool, key: &F) -> io::Result<Chunk>
where
    F: Fn(&String) -> K,
    K: Ord,
{
    if chunk.rough_count == RoughCount::Zero || chunk.rough_count == RoughCount::One {
        return Ok(chunk);
    }

    if chunk.fit_in_buffer() {
        return chunk.sort(rev, key);
    }

    let (c1, c2) = chunk.split()?;

    if c2.rough_count == RoughCount::Zero {
        return c1.sort(rev, key);
    }

    Ok(merge(
        sort_chunk(c1, rev, key)?,
        sort_chunk(c2, rev, key)?,
        rev,
        key,
    )?)
}

fn merge<F, K>(c1: Chunk, c2: Chunk, rev: bool, key: &F) -> io::Result<Chunk>
where
    F: Fn(&String) -> K,
    K: Ord,
{
    assert!(c1.capacity == c2.capacity);

    let mut reader1 = io::BufReader::new(&c1.file);
    let mut reader2 = io::BufReader::new(&c2.file);
    let mut writer = io::BufWriter::new(tempfile::tempfile()?);
    let mut r1_buf = String::new();
    let mut r2_buf = String::new();

    let mut r1_read = reader1.read_line(&mut r1_buf)?;
    let mut r2_read = reader2.read_line(&mut r2_buf)?;

    let mut r1_key = key(&r1_buf);
    let mut r2_key = key(&r2_buf);

    while r1_read > 0 && r2_read > 0 {
        if (!rev && r1_key < r2_key) || (rev && r1_key > r2_key) {
            writer.write(&r1_buf.as_bytes())?;
            r1_buf.clear();
            r1_read = reader1.read_line(&mut r1_buf)?;

            if r1_read > 0 {
                r1_key = key(&r1_buf);
            }
        } else {
            writer.write(&r2_buf.as_bytes())?;
            r2_buf.clear();
            r2_read = reader2.read_line(&mut r2_buf)?;

            if r2_read > 0 {
                r2_key = key(&r2_buf);
            }
        }
    }

    while r1_read > 0 {
        writer.write(&r1_buf.as_bytes())?;
        r1_buf.clear();
        r1_read = reader1.read_line(&mut r1_buf)?;
    }

    while r2_read > 0 {
        writer.write(&r2_buf.as_bytes())?;
        r2_buf.clear();
        r2_read = reader2.read_line(&mut r2_buf)?;
    }

    let cap = c1.capacity;
    Ok(Chunk::new(writer.into_inner()?, cap)?)
}
