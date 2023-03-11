//! C. Choosing flowers
//! https://codeforces.com/problemset/status/1379/problem/C?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1379C

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    /// 只有一行会选到b，其他的都只选a。双指针
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let mut n: usize = scanner.next();
            let m: usize = scanner.next();
            let mut ba: Vec<(usize, usize)> = Vec::with_capacity(m);
            let mut a: Vec<usize> = Vec::with_capacity(m);
            for _ in 0..m {
                let xa: usize = scanner.next();
                let xb: usize = scanner.next();
                ba.push((xb, xa));
                a.push(xa);
            }
            ba.sort_unstable();
            a.sort_unstable();
            let mut result = 0;
            let mut i = m;
            let mut sa = 0;
            for (xb, xa) in ba.into_iter().rev() {
                while n > 0 && i > 0 && a[i - 1] > xb {
                    sa += a[i-1];
                    i -= 1;
                    n -= 1;
                }
                let mut nn = n;
                let mut s = sa;
                if nn > 0 {
                    if xa < xb {
                        nn -= 1;
                        s += xa;
                    }
                    s += nn * xb;
                }
                result = result.max(s);
            }
            puts!("{}", result);
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
    t.test("2\n4 3\n5 0\n1 4\n2 2\n\n5 3\n5 2\n4 2\n3 1\n",
           "14\n16\n");
}
