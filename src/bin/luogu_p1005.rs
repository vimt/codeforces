//! [NOIP2007 提高组] 矩阵取数游戏
//! https://www.luogu.com.cn/problem/P1005
//! https://www.luogu.com.cn/problem/solution/P1005

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let m: usize = scanner.next();
        let n: usize = scanner.next();
        let mut result = 0;
        for _ in 0..m {
            let a: Vec<u128> = (0..n).map(|_| scanner.next()).collect();
            let mut dp = vec![vec![0; n + 1]; n + 1];
            for i in 1..=n {  // 取了i个数
                for j in 0..=i { // 左边取了j个数
                    if j > 0 { dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + (a[j - 1] << i)); }
                    let right = i - j;
                    if j < i {
                        dp[i][j] = dp[i][j].max(dp[i - 1][j] + (a[n - right] << i));
                    }
                }
            }
            result += *dp[n].iter().max().unwrap();
        }

        puts!("{}", result);
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
            "2 3\n1 2 3\n3 4 2\n",
            "82\n",
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
