//! B. Tolik and His Uncle
//! https://codeforces.com/problemset/status/1179/problem/B
//! https://www.luogu.com.cn/problem/solution/CF1179B

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let m: usize = scanner.next();
        let n: usize = scanner.next();
        for i in 1..=m / 2 {
            for j in 1..=n {
                puts!("{} {}", i, j);
                puts!("{} {}", m+1-i, n+1-j);
            }
        }
        if m & 1 == 1 {
            let i = (m + 1) / 2;
            for j in 1..=n / 2 {
                puts!("{} {}", i, j);
                puts!("{} {}", i, n+1-j);
            }
            if n & 1 == 1 {
                puts!("{} {}", i, (n+1)/2);
            }
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
    use codeforces::input;
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "3 3\n",
            "",
            "2 3\n",
            "1 1\n1 3\n1 2\n2 2\n2 3\n2 1\n",
            "1 1\n",
            "1 1\n",
        ];
        let functions: Vec<fn(&mut Scanner<_>, &mut _)> = vec![
            my::solve,
        ];
        for func in functions {
            for tc in testcases.chunks(2) {
                let mut output = Vec::new();
                func(&mut Scanner::new(tc[0].as_bytes()), &mut output);
                let mut sc = Scanner::new(tc[0].as_bytes());
                input! {sc, m: usize, n: usize}
                sc = Scanner::new(&output);
                input! {sc, pos:[(usize,usize); m*n]}
                let mut vis = vec![vec![0; n]; m];
                for (x, y) in pos {
                    vis[x - 1][y - 1] += 1;
                    assert_eq!(vis[x - 1][y - 1], 1, "{} {}, output: {}", x, y, std::str::from_utf8(&output).unwrap());
                }
            }
        }
    }
}
