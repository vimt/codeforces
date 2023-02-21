//! [NOIP2005 提高组] 过河
//! https://www.luogu.com.cn/record/list?status=&pid=P1052&page=1&orderBy=1

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let len: usize = scanner.next();
        let s: usize = scanner.next();
        let t: usize = scanner.next();
        let m: usize = scanner.next();
        let mut stones: Vec<usize> = (0..m).map(|_| scanner.next()).collect();
        stones.push(len);
        stones.sort_unstable();
        let mut new_stone = vec![0; 2 * t * stones.len()];
        let mut j = 0;
        // 对于 1e9 的输入，和小t，需要处理
        for i in 0..stones.len() {
            let d = stones[i] - if i > 0 { stones[i - 1] } else { 0 };
            if d > t {
                j += d % t + t;
            } else {
                j += d;
            }
            new_stone[j] = 1;
        }
        new_stone[j] = 0;
        let mut dp = vec![i32::MAX / 2; j + t];
        dp[0] = 0;
        for i in 1..j + t {
            for step in s..=t {
                if i >= step {
                    dp[i] = dp[i].min(dp[i - step] + new_stone[i]);
                }
            }
        }
        puts!("{}", dp[j..j + t].iter().min().unwrap());
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
            "10\n2 3 5\n2 3 5 6 7\n",
            "2\n",
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
