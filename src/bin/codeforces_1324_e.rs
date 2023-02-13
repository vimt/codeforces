//! https://codeforces.com/contest/1324/problem/E
//! E. Sleeping Schedule

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use std::mem::swap;
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
        swap
        puts!("{}", dp[len].iter().max().unwrap());
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
            "7 24 21 23\n16 17 14 20 20 11 22\n",
            "3\n",
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
