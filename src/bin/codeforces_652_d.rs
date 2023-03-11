//! D. Nested Segments
//! https://codeforces.com/problemset/status/652/problem/D?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF652D

use codeforces::scanner::Scanner;

mod my {
    use std::collections::HashMap;
    use std::io::{BufRead, Write};
    use super::*;

    struct Fenwick {
        arr: Vec<i32>,
    }

    fn lowbit(a: i32) -> usize {
        (-a & a) as usize
    }

    impl Fenwick {
        fn new(n: usize) -> Self {
            Self { arr: vec![0; n] }
        }
        fn add(&mut self, mut i: usize, delta: i32) {
            while i < self.arr.len() {
                self.arr[i] += delta;
                i += lowbit(i as i32);
            }
        }
        fn sum(&self, mut k: usize) -> i32 {
            let mut ans = 0;
            while k > 0 {
                ans += self.arr[k];
                k -= lowbit(k as i32);
            }
            ans
        }
    }

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let n: usize = scanner.next();
        // 离散化
        let mut discretize = Vec::with_capacity(n);
        let mut ranges = Vec::with_capacity(n);
        for i in 0..n {
            let a: i32 = scanner.next();
            let b: i32 = scanner.next();
            discretize.push(b);
            ranges.push((-a, b, i));
        }
        ranges.sort_unstable();
        discretize.sort_unstable();
        let m: HashMap<i32, usize> = discretize.into_iter().zip(1..).collect();
        let mut pre_right = Fenwick::new(n + 1);
        let mut result = vec![0; n];
        for (_, t, i) in ranges {
            let ti = m[&t];
            result[i] = pre_right.sum(ti);
            pre_right.add(ti, 1);
        }
        for num in result {
            puts!("{}", num);
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
    t.test("4\n1 8\n2 3\n4 7\n5 6\n",
           "3\n0\n1\n0\n");
    t.test("3\n3 4\n1 5\n2 6\n",
           "0\n1\n1\n");
}
