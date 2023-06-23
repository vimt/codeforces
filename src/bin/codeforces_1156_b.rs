//! B. Ugly Pairs
//! https://codeforces.com/problemset/status/1156/problem/B?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1156B

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let n: usize = scanner.next();
        for _ in 0..n {
            let s = scanner.next::<String>().into_bytes();
            let mut cnt = [0; 26];
            for &ch in &s {
                cnt[(ch - b'a') as usize] += 1;
            }
            let mut a = Vec::with_capacity(s.len());
            let mut b = Vec::with_capacity(s.len());
            for i in 0..26 {
                let t = if i & 1 == 0 { &mut a } else { &mut b };
                for _ in 0..cnt[i] {
                    t.push(i as u8 + b'a');
                }
            }
            fn to_string(x: Vec<u8>) -> String { unsafe { String::from_utf8_unchecked(x) } }
            if a.is_empty() {
                puts!("{}", to_string(b));
            } else if b.is_empty() {
                puts!("{}", to_string(a));
            } else if a[0].abs_diff(*b.last().unwrap()) != 1 {
                puts!("{}{}", to_string(b), to_string(a));
            } else if b[0].abs_diff(*a.last().unwrap()) != 1 {
                puts!("{}{}", to_string(a), to_string(b));
            } else {
                puts!("No answer");
            }
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
    use codeforces::tester::{Tester, Testcase};
    fn verify(mut tc: Testcase) {
        for _ in 0..tc.input_scanner().next::<i32>() {
            let a = tc.input_scanner().next::<String>().into_bytes().into_iter().fold([0; 26], |mut cnt, x| {
                cnt[(x - b'a') as usize] += 1;
                cnt
            });
            let expect = tc.expect_scanner().next::<String>();
            if expect == "No" {
                let output = tc.output_scanner().next::<String>();
                tc.assert_eq(output, expect);
                tc.output_scanner().next::<String>();
                tc.expect_scanner().next::<String>();
            } else {
                let output = tc.output_scanner().next::<String>().into_bytes();
                let b = output.iter().fold([0; 26], |mut cnt, &x| {
                    cnt[(x - b'a') as usize] += 1;
                    cnt
                });
                tc.assert_eq(a, b);
                for i in 1..output.len() {
                    tc.assert_ne(output[i - 1].abs_diff(output[i]), 1);
                }
            }
        }
    }
    let t = Tester::with_judge(verify, solves!(my));
    t.test("4\nabcd\ngg\ncodeforces\nabaca\n",
           "cadb\ngg\ncodfoerces\nNo answer\n");
}
