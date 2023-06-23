//! C. Sum in Binary Tree
//! https://codeforces.com/problemset/status/1843/problem/C?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1843C

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let mut a: i64 = scanner.next();
            let mut result = 0;
            while a > 0 {
                result += a;
                a /= 2;
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
    t.test("6\n3\n10\n37\n1\n10000000000000000\n15",
"4\n18\n71\n1\n19999999999999980\n26\n");
}
