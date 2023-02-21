//! E. Nearest Opposite Parity
//! https://codeforces.com/contest/1272/problem/E
//! https://codeforces.com/problemset/status/1272/problem/E

use codeforces::scanner::Scanner;

mod my {
    use std::collections::VecDeque;
    use std::io::{BufRead, Write};
    use codeforces::adjacency::Adjacency;
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        let len: usize = scanner.next();
        let a: Vec<usize> = (0..len).map(|_| scanner.next()).collect();
        let mut g = Adjacency::new(len, len * 2);
        for (i, &num) in a.iter().enumerate() {
            if i >= num { g.add(i - num, i); }
            if i + num < len { g.add(i + num, i); }
        }
        let mut step = vec![[i32::MAX, i32::MAX]; len];
        let mut q = VecDeque::with_capacity(3 * len);
        for i in 0..len {
            step[i][a[i] & 1] = 0;
            q.push_back((i, a[i] & 1, 0));
        }
        while !q.is_empty() {
            let (i, o, level) = q.pop_front().unwrap();
            for j in g.neigh(i) {
                if level + 1 < step[j][o] {
                    step[j][o] = level + 1;
                    q.push_back((j, o, level + 1));
                }
            }
        }
        for (i, num) in a.into_iter().enumerate() {
            let s = step[i][num & 1 ^ 1];
            let _ = write!(out, "{}{}", if s == i32::MAX { -1 } else { s }, if i + 1 < len { " " } else { "\n" });
        }
    }
}

fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    my::solve(&mut Scanner::new(stdin), &mut stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "10\n4 5 7 6 7 5 4 4 6 4\n",
            "1 1 1 2 -1 1 1 3 1 1\n",
        ];
        let functions: Vec<fn(&mut Scanner<_>, &mut _)> = vec![
            my::solve,
        ];
        for func in functions {
            for tc in testcases.chunks(2) {
                let mut output = Vec::new();
                func(&mut Scanner::new(tc[0].as_bytes()), &mut output);
                assert_eq!(String::from_utf8(output).unwrap(), tc[1], "input: {}", tc[0]);
            }
        }
    }
}
