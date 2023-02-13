//! https://codeforces.com/contest/1324/problem/C
//! C. Frog Jumps

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let s: String = scanner.next();
            let mut s = s.into_bytes();
            s.push(b'R');
            let mut result = 0;
            let mut last = 0;
            for (i, &ch) in s.iter().enumerate() {
                if ch == b'R' {
                    result = result.max(i - last + 1);
                    last = i + 1;
                }
            }
            puts!("{}", result);
        };
        for _ in 0..t {
            go();
        }
    }
}

fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    my::solve(Scanner::new(stdin), &mut stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "6\nLRLRRLL\nL\nLLR\nRRRR\nLLLLLL\nR\n",
            "3\n2\n3\n1\n7\n1\n",
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
            my::solve,
        ];
        for func in functions {
            for tc in testcases.chunks(2) {
                let mut output = Vec::new();
                func(Scanner::new(tc[0].as_bytes()), &mut output);
                assert_eq!(String::from_utf8(output).unwrap(), tc[1], "input: {}", tc[0]);
            }
        }
    }
}
