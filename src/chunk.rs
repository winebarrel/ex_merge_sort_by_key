use super::file_utils;
use file_utils::RoughCount;
use io::prelude::BufRead;
use std::fs;
use std::io;
use std::io::Write;

pub(super) struct Chunk {
    pub(super) file: fs::File,
    pub(super) capacity: u64,
    pub(super) rough_count: file_utils::RoughCount,
}

impl Chunk {
    pub(super) fn new(f: fs::File, cap: u64) -> io::Result<Chunk> {
        let rc = file_utils::count_roughly(&f)?;

        Ok(Chunk {
            capacity: cap,
            file: f,
            rough_count: rc,
        })
    }

    pub(super) fn fit_in_buffer(&self) -> bool {
        self.file.metadata().unwrap().len() <= self.capacity
    }

    pub(super) fn sort<F, K>(&self, key: &F) -> io::Result<Chunk>
    where
        F: Fn(&String) -> K,
        K: Ord,
    {
        let mut reader = io::BufReader::new(&self.file);
        let mut lines = vec![];
        let mut buf = String::new();

        while reader.read_line(&mut buf)? > 0 {
            lines.push(buf.clone());
            buf.clear();
        }

        lines.sort_by_cached_key(key);
        let mut writer = io::BufWriter::new(tempfile::tempfile()?);

        for l in lines {
            writer.write(l.as_bytes())?;
        }

        let file = writer.into_inner()?;
        file_utils::rewind(&file)?;
        Chunk::new(file, self.capacity)
    }

    pub(super) fn split(&self) -> io::Result<(Chunk, Chunk)> {
        assert!(self.rough_count == RoughCount::Two || self.rough_count == RoughCount::ThreeOrMore);

        let mid = self.file.metadata().unwrap().len() / 2;
        let mut reader = io::BufReader::new(&self.file);
        let mut writer1 = io::BufWriter::new(tempfile::tempfile().unwrap());
        let mut writer2 = io::BufWriter::new(tempfile::tempfile().unwrap());
        let mut sum = 0;
        let mut buf = String::new();

        while reader.read_line(&mut buf)? > 0 {
            sum += buf.len() as u64;
            writer1.write(buf.as_bytes())?;
            buf.clear();

            if sum >= mid || self.rough_count == RoughCount::Two {
                break;
            }
        }

        while reader.read_line(&mut buf)? > 0 {
            writer2.write(buf.as_bytes())?;
            buf.clear();
        }

        let file1 = writer1.into_inner()?;
        file_utils::rewind(&file1)?;
        let file2 = writer2.into_inner()?;
        file_utils::rewind(&file2)?;

        Ok((
            Chunk::new(file1, self.capacity)?,
            Chunk::new(file2, self.capacity)?,
        ))
    }
}
