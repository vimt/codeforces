//! B. Long Long
//! https://codeforces.com/problemset/status/1843/problem/B?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1843B

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let a: Vec<i64> = (0..len).map(|_| scanner.next()).collect();
            let mut pos = 0;
            let mut neg = 0;
            let mut i = 0;
            while i < len {
                if a[i] == 0 {
                    i += 1;
                    continue;
                }
                let mut j = i + 1;
                while j < len && (a[j] == 0 || (a[j] < 0) == (a[i] < 0)) {
                    j += 1;
                }
                if a[i] < 0 { neg += 1; } else { pos += 1; }
                i = j;
            }
            puts!("{} {}", a.iter().map(|x| x.abs()).sum::<i64>(), neg.min(pos + 1));
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
    t.test("5\n6\n-1 7 -4 -2 5 -8\n8\n-1 0 0 -2 1 0 -3 0\n5\n2 -1 0 -3 -7\n5\n0 -17 0 1 0\n4\n-1 0 -2 -1",
           "27 3\n7 2\n13 1\n18 1\n4 1\n");
}
