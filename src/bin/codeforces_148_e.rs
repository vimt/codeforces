//! E. Porcelain
//! https://codeforces.com/problemset/status/148/problem/E
//! https://www.luogu.com.cn/problem/solution/CF148E
//! n行双端队列，n行总共选m个数 和的最大值

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    /// 预处理+多重背包
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let n: usize = scanner.next();
        let m: usize = scanner.next();
        let mut dp = vec![0; m + 1];
        let mut b = 0;
        for _ in 0..n {
            let k: usize = scanner.next();
            let mut presum = vec![0; k + 1];
            for i in 0..k {
                presum[i + 1] = scanner.next::<i32>() + presum[i];
            }
            let mut select = vec![0; k + 1]; // 这一行选k个的最大值
            for i in 1..=k {
                for j in 0..=i { // 前面选j个，后面选i-j个
                    select[i] = select[i].max(presum[j] + presum[k] - presum[k + j - i]);
                }
            }
            b = (b + k).min(m);// 优化循环上界
            for i in (1..=b).rev() { // 前x行选i个
                for j in 1..=k.min(i) {// 这一行选[1,k]个
                    dp[i] = dp[i].max(dp[i - j] + select[j]);
                }
            }
        }
        puts!("{}", dp[m]);
    }
}

#[cfg(not(debug))]
fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    my::solve(&mut Scanner::new(stdin), &mut stdout)
}

#[cfg(debug)]
fn main() {
    use codeforces::solves;
    use codeforces::tester::Tester;
    let t = Tester::new(solves!(my));
    t.test("2 3\n3 3 7 2\n3 4 1 5\n",
           "15\n");
    t.test("1 3\n4 4 3 1 2\n",
           "9\n");
}
