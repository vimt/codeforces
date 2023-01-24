use std::io::prelude::*;
use crate::scanner::Scanner;

pub struct Tester<R> {
    func: fn(Scanner<R>, &mut Vec<u8>),
}

impl<R: BufRead> Tester<R> {
    pub fn new(func: fn(Scanner<R>, &mut Vec<u8>)) -> Self {
        Self {func}
    }
    pub fn run_test(&self, input: R, expect: &[u8]) {
        let mut output = Vec::new();
        (self.func)(Scanner::new(input), &mut output);
        assert_eq!(output, expect);
    }
}
