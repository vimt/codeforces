//! C. Fancy Number
//! https://codeforces.com/problemset/status/118/problem/C?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF118C

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let _: usize = scanner.next();
        let k: usize = scanner.next();
        let mut cnt = vec![vec![]; 10];
        let s = scanner.next::<String>().into_bytes();
        for (i, &ch) in s.iter().enumerate() {
            cnt[(ch - b'0') as usize].push(i);
        }
        if cnt.iter().map(|x| x.len()).max().unwrap() >= k {
            puts!("0");
            puts!("{}", unsafe {std::str::from_utf8_unchecked(&s)});
            return;
        }
        let mut min_cost = i32::MAX;
        let mut result = vec![];
        for to_ch in b'0'..=b'9' {
            let mut need = k - cnt[(to_ch - b'0') as usize].len();
            let mut cost = 0;
            let mut s = s.clone();
            for other in 1..=9 {
                let cha = to_ch + other;
                if cha <= b'9' {
                    let remove = need.min(cnt[(cha - b'0') as usize].len());
                    cost += remove as i32 * other as i32;
                    need -= remove;
                    for &i in &cnt[(cha - b'0') as usize][..remove] {
                        s[i] = to_ch;
                    }
                }
                let chb = to_ch - other;
                if need > 0 && chb >= b'0' {
                    let remove = need.min(cnt[(chb - b'0') as usize].len());
                    cost += remove as i32 * other as i32;
                    need -= remove;
                    for &i in cnt[(chb - b'0') as usize].iter().rev().take(remove) {
                        s[i] = to_ch;
                    }
                }
                if need == 0 { break; }
            }
            if cost < min_cost || (cost == min_cost && s < result) {
                min_cost = cost;
                result = s;
            }
        }
        puts!("{}", min_cost);
        puts!("{}", unsafe {std::str::from_utf8_unchecked(&result)});
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
    t.test("6 5\n898196\n",
           "4\n888188\n");
    t.test("3 2\n533\n",
           "0\n533\n");
    t.test("10 6\n0001112223\n",
           "3\n0000002223\n");
    t.test("16 14\n6124258626539246",
           "22\n4444448444449444\n");
    t.test("8 4\n22294777",
           "2\n22274777\n");
}
