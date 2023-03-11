//! D. Armchairs
//! https://codeforces.com/problemset/status/1525/problem/D
//! https://www.luogu.com.cn/problem/solution/CF1525D
//! https://codeforces.com/contest/1525/problem/D
//! 一个人从 i 移动到 j 的耗时为 abs(i-j)。所有人都坐到椅子上，耗时之和最小是多少？

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    /// dp[i][j] 表示前i个椅子，坐j个人的最小代价
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let len: usize = scanner.next();
        let mut people = Vec::with_capacity(len);
        let mut chair = Vec::with_capacity(len);
        for i in 0..len {
            let pos: i32 = scanner.next();
            if pos == 1 {
                people.push(i as i32); // 人
            } else {
                chair.push(i as i32); // 椅子
            }
        }
        let mut dp = vec![i32::MAX / 2; people.len() + 1];
        dp[0] = 0;
        for (i, c) in chair.into_iter().enumerate() {
            for j in (0..people.len().min(i + 1)).rev() {
                dp[j + 1] = dp[j + 1].min(dp[j] + (c - people[j]).abs());
            }
        }
        puts!("{}", dp[people.len()]);
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
    t.test("7\n1 0 0 1 0 0 1\n",
           "3\n");
    t.test("6\n1 1 1 0 0 0\n",
           "9\n");
    t.test("5\n0 0 0 0 0\n",
           "0\n");
}
