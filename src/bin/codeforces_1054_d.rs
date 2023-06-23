//! D. Changing Array
//! https://codeforces.com/problemset/status/1054/problem/D?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1054D
//! 对数组的随便n个数 ^ mask, 求异或后的 最多有多少个子数组异或和不为0

use codeforces::scanner::Scanner;

mod my {
    use std::collections::HashMap;
    use std::io::{BufRead, Write};
    use super::*;

    /// https://www.luogu.com.cn/blog/endlesscheng/solution-cf1054d
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        fn c2(num: i64) -> i64 { num * (num - 1) / 2 }
        let n: usize = scanner.next();
        let k: i32 = scanner.next();
        let mut cnt: HashMap<i64, i64> = HashMap::with_capacity(n);
        cnt.insert(0, 1);
        let mask = (1i64 << k) - 1;
        let mut cur = 0;
        for _ in 0..n {
            let num: i64 = scanner.next();
            cur ^= num;
            *cnt.entry(cur.min(cur ^ mask)).or_default() += 1;
        }
        let mut result = c2(n as i64 + 1);
        for (_, v) in cnt {
            result -= c2(v / 2) + c2(v - v / 2);
        }
        puts!("{}", result);
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
    t.test("3 2\n1 3 0\n",
           "5\n");
    t.test("6 3\n1 4 4 7 3 4\n",
           "19\n");
}
