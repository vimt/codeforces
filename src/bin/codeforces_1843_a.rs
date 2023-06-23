//! A. Sasha and Array Coloring
//! https://codeforces.com/problemset/status/1843/problem/A?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1843A

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let mut a: Vec<i32> = (0..len).map(|_| scanner.next()).collect();
            a.sort_unstable();
            let mut result = 0;
            for i in 0..a.len() / 2 {
                result += a[len - 1 - i] - a[i];
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
    t.test("6\n5\n1 5 6 3 4\n1\n5\n4\n1 6 3 9\n6\n1 13 9 3 7 2\n4\n2 2 2 2\n5\n4 5 2 2 3",
           "7\n0\n11\n23\n0\n5\n");
}
