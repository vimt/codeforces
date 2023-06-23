//! E. Tracking Segments
//! https://codeforces.com/problemset/status/1843/problem/E?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1843E

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let n: usize = scanner.next();
            let m: usize = scanner.next();
            let segments: Vec<(usize, usize)> = (0..m).map(|_| (scanner.next(), scanner.next())).collect();
            let qlen: usize = scanner.next();
            let q: Vec<usize> = (0..qlen).map(|_| scanner.next()).collect();
            let mut left = 1;
            let mut right = qlen + 1;
            while left < right {
                let mid = (left + right) / 2;
                let mut data = vec![0; n];
                for i in 0..mid {
                    data[q[i] - 1] = 1;
                }
                let mut presum = vec![0; n + 1];
                for i in 0..n {
                    presum[i + 1] = presum[i] + data[i];
                }
                let mut ok = false;
                for &(a, b) in &segments {
                    if presum[b] - presum[a - 1] > (b + 1 - a) / 2 {
                        ok = true;
                        break;
                    }
                }
                if ok {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            puts!("{}", if left == qlen + 1 {-1} else {left as i64});
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
    t.test("6\n5 5\n1 2\n4 5\n1 5\n1 3\n2 4\n5\n5\n3\n1\n2\n4\n4 2\n1 1\n4 4\n2\n2\n3\n5 2\n1 5\n1 5\n4\n2\n1\n3\n4\n5 2\n1 5\n1 3\n5\n4\n1\n2\n3\n5\n5 5\n1 5\n1 5\n1 5\n1 5\n1 4\n3\n1\n4\n3\n3 2\n2 2\n1 3\n3\n2\n3\n1",
           "3\n-1\n3\n3\n3\n1\n");
}
