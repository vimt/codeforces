//! https://codeforces.com/contest/1324/problem/B
//! B. Yet Another Palindrome Problem

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let mut first_seen = vec![None; len + 1];
            let a: Vec<usize> = (0..len).map(|_| scanner.next()).collect();
            for i in 0..len {
                if let Some(j) = first_seen[a[i]] {
                    if i - j > 1 {
                        puts!("YES");
                        return;
                    }
                } else {
                    first_seen[a[i]] = Some(i);
                }
            }
            puts!("NO");
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
            "5\n3\n1 2 1\n5\n1 2 2 3 2\n3\n1 1 2\n4\n1 2 2 1\n10\n1 1 2 2 3 3 4 4 5 5\n",
            "YES\nYES\nNO\nYES\nNO\n",
            "1\n3\n3 3 3",
            "YES\n",
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
