#![feature(portable_simd)]
//! C. The Values You Can Make
//! https://codeforces.com/problemset/status/687/problem/C?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF687C
//! 从C选一些数，组成A，满足sum(A)=k，再从A选一些数组成B，求所有可能的sum(B)

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    /// dp[i][j][k-j] 表示前i个数，选一些数组成B，和为j，选一些数组成A-B，和为k-j，看能不能组
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let n: usize = scanner.next();
        let k: usize = scanner.next();
        let c: Vec<usize> = (0..n).map(|_| scanner.next()).collect();
        let mut dp = vec![vec![false; k + 1]; k + 1];
        dp[0][0] = true;
        for num in c {
            if num > k { continue; }
            for j in (0..=k).rev() {
                for c in (0..=k).rev() {
                    if j >= num && !dp[j][c] {
                        dp[j][c] = dp[j - num][c];
                    }
                    if c >= num && !dp[j][c] {
                        dp[j][c] = dp[j][c - num];
                    }
                }
            }
        }
        let result: Vec<usize> = (0..=k).filter(|&x| dp[x][k - x]).collect();
        puts!("{}", result.len());
        for num in result {
            let _ = write!(out, "{} ", num);
        }
        puts!("");
    }
}

mod num1 {
    use std::io::{BufRead, Write};
    use std::ops::Shl;
    use std::simd::u64x8;
    use super::*;

    /// dp[x][i]表示 前x个数 组成和为i的 的子集有哪些。
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let n: usize = scanner.next();
        let k: usize = scanner.next();
        let c: Vec<usize> = (0..n).map(|_| scanner.next()).collect();
        let zero = u64x8::from([0; 8]);
        let mut dp = vec![zero; k + 1];
        dp[0].as_mut_array()[0] = 1;
        let mut used = vec![false; k + 1];
        used[0] = true;
        for num in c {
            let mut n = [0; 8];
            n[0] += num as u64;
            let sn = u64x8::from(n);
            for j in (num..=k).rev() {
                if used[j - num] {
                    used[j] = true;
                    let x = dp[j - num];
                    dp[j] |= x;
                    dp[j] |= x.shl(sn);
                }
            }
        }
        let a = dp[k].as_array();
        let result: Vec<usize> = (0..=k).filter(|&x| a[x >> 6] >> (x & 63) & 1 == 1).collect();
        puts!("{}", result.len());
        for num in result {
            let _ = write!(out, "{} ", num);
        }
        puts!("");
    }
}

#[cfg(not(debug))]
fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    num1::solve(&mut Scanner::new(stdin), &mut stdout)
}

#[cfg(debug)]
fn main() {
    use codeforces::solves;
    use codeforces::tester::Tester;
    let t = Tester::new(solves!(my, num1));
    t.test("6 18\n5 6 1 10 12 2\n",
           "16\n0 1 2 3 5 6 7 8 10 11 12 13 15 16 17 18 \n");
    t.test("3 50\n25 25 50\n",
           "3\n0 25 50 \n");
}
