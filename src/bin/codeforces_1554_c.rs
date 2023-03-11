//! C. Mikasa
//! https://codeforces.com/problemset/status/1554/problem/C
//! https://www.luogu.com.cn/problem/solution/CF1554C
//! a = [n xor 0, n xor 1, n xor 2, ..., n xor m] 输出不在 a 中的最小非负整数。

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    /// 构造题：设答案为x，n ^ [0..=m] = a，那么 n ^ a = [0..=m]
    /// 那么要构造一个x，让 n ^ x >= m+1
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let n: usize = scanner.next();
            let m: usize = scanner.next();
            let mm = m + 1;
            let mut x = 0;
            for i in (0..31).rev() {
                if n >> i & 1 == 1 {
                    if mm >> i & 1 == 0 {
                        break;
                    }
                } else {
                    if mm >> i & 1 == 1 {
                        x |= 1 << i;
                    }
                }
            }
            puts!("{}", x);
        };
        for _ in 0..t {
            go();
        }
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
    t.test("5\n3 5\n4 6\n3 2\n69 696\n123456 654321\n",
           "4\n3\n0\n640\n530866\n");
}
