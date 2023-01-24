use std::io::prelude::*;
use std::str::FromStr;

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    pub fn token<T: FromStr>(&mut self) -> T where <T as FromStr>::Err: std::fmt::Debug {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().unwrap();
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).unwrap();
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}
