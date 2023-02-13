//! https://codeforces.com/contest/1324/problem/A
//! A. Yet Another Tetris Problem

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let mut a: Vec<i32> = vec![0; len];
            let mut max = 0;
            for i in 0..len {
                a[i] = scanner.next();
                max = max.max(a[i]);
            }
            for num in a {
                if (max - num) & 1 == 1 {
                    puts!("NO");
                    return;
                }
            }
            puts!("YES");
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
            "4\n3\n1 1 3\n4\n1 1 2 1\n2\n11 11\n1\n100\n",
            "YES\nNO\nYES\nYES\n",
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
