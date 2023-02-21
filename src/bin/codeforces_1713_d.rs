//! D. Tournament Countdown
//! https://codeforces.com/contest/1713/problem/D
//! https://codeforces.com/problemset/status/1713/problem/D

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*);let _ = out.flush(););}
        macro_rules! ask {($a:expr, $b:expr) => {{puts!("? {} {}", $a, $b);scanner.next::<i8>()}};}
        let t: usize = scanner.next();
        let mut go = || {
            let n: usize = scanner.next();
            let mut a: Vec<usize> = (1..=1 << n).collect();
            while a.len() != 1 {
                let mut tmp = Vec::with_capacity(a.len() / 2);
                if a.len() == 2 {
                    tmp.push(if ask!(a[0], a[1]) == 1 { a[0] } else { a[1] })
                } else {
                    let mut i = 0;
                    while i < a.len() {
                        let q = ask!(a[i], a[i + 2]);
                        match q {
                            0 => {
                                tmp.push(a[i + 1]);
                                tmp.push(a[i + 3]);
                            }
                            1 => {
                                tmp.push(a[i]);
                                tmp.push(a[i + 3]);
                            }
                            2 => {
                                tmp.push(a[i + 2]);
                                tmp.push(a[i + 1]);
                            }
                            _ => unreachable!()
                        }
                        i += 4;
                    }
                }
                a = tmp;
            }
            puts!("! {}", a[0]);
        };
        for _ in 0..t {
            go();
        }
    }
}

fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    my::solve(&mut Scanner::new(stdin), &mut stdout)
}
