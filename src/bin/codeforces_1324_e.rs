//! https://codeforces.com/contest/1324/problem/E
//! E. Sleeping Schedule

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let len: usize = scanner.next();
        let h: i32 = scanner.next();
        let l: i32 = scanner.next();
        let r: i32 = scanner.next();
        let a: Vec<i32> = (0..len).map(|_| scanner.next()).collect();
        let mut dp = vec![vec![i32::MIN / 2; h as usize]; len + 1];
        dp[0][0] = 0;
        for i in 0..len {
            for j in 0..h {
                let p1 = (j - a[i] % h + h) % h;
                let p2 = (j - a[i] % h + 1 + h) % h;
                dp[i + 1][j as usize] = dp[i][p1 as usize].max(dp[i][p2 as usize]) + (j >= l && j <= r) as i32;
            }
        }
        puts!("{}", dp[len].iter().max().unwrap());
    }
}


mod num1 {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let len: usize = scanner.next();
        let h: i32 = scanner.next();
        let l: i32 = scanner.next();
        let r: i32 = scanner.next();
        let a: Vec<i32> = (0..len).map(|_| scanner.next()).collect();
        let presum: Vec<i32> = a.iter().scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        }).collect();
        let mut f = vec![0; len + 1];
        let mut g = vec![0; len + 1];
        for i in 0..len {
            for j in 0..=i + 1 {
                let k = (presum[i] - j as i32) % h;
                g[j] = f[j];
                if j > 0 {
                    g[j] = g[j].max(f[j - 1]);
                }
                if k >= l && k <= r {
                    g[j] += 1;
                }
            }
            std::mem::swap(&mut f, &mut g);
        }
        puts!("{}", f.iter().max().unwrap());
    }
}


fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    num1::solve(Scanner::new(stdin), &mut stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "7 24 21 23\n16 17 14 20 20 11 22\n",
            "3\n",
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
            num1::solve,
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
