//! https://codeforces.com/contest/1374/problem/E1
//! E1. Reading Books (easy version)

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let len: usize = scanner.next();
        let k: usize = scanner.next();
        let mut a = Vec::with_capacity(len);
        let mut b = Vec::with_capacity(len);
        let mut both = Vec::with_capacity(len);
        for _ in 0..len {
            let (t, al, bl) = (scanner.next::<i32>(), scanner.next::<usize>(), scanner.next::<usize>());
            match (al, bl) {
                (1, 1) => both.push(t),
                (1, 0) => a.push(t),
                (0, 1) => b.push(t),
                (_, _) => {}
            }
        }
        if a.len().min(b.len()) + both.len() < k {
            puts!("-1");
            return;
        }
        a.sort_unstable();
        b.sort_unstable();
        both.extend(a.into_iter().zip(b).map(|(a, b)| a + b));
        both.sort_unstable();
        puts!("{}", both[..k].iter().sum::<i32>());
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
            "8 4\n7 1 1\n2 1 1\n4 0 1\n8 1 1\n1 0 1\n1 1 1\n1 0 1\n3 0 0\n",
            "18\n",
            "5 2\n6 0 0\n9 0 0\n1 0 1\n2 1 1\n5 1 0\n",
            "8\n",
            "5 3\n3 0 0\n2 1 0\n3 1 0\n5 0 1\n3 0 1\n",
            "-1\n",
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
