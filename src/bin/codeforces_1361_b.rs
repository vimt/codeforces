//! B. Johnny and Grandmaster

use codeforces::scanner::Scanner;

const MOD: i64 = 1e9 as i64 + 7;


pub fn quick_pow(mut base: i64, mut pow: i64, mod0: i64) -> i64 {
    base = base % mod0;
    let mut ans = 1;
    while pow != 0 {
        if pow & 1 == 1 {
            ans = ans * base % mod0;
        }
        base = base * base % mod0;
        pow >>= 1;
    }
    ans
}


mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        'o: for _ in 0..t {
            let len: usize = scanner.next();
            let p: i64 = scanner.next();
            let mut a: Vec<i64> = (0..len).map(|_| scanner.next()).collect();
            if p == 1 {
                puts!("{}", len & 1);
                continue;
            }
            a.sort_unstable();
            let mut target_k = a.pop().unwrap();
            let mut st: Vec<(i64, i64)> = vec![];
            while !a.is_empty() {
                let mut k = a.pop().unwrap();
                while !st.is_empty() && *st.last().unwrap() == (k, p - 1) {
                    st.pop();
                    k += 1;
                }
                if k == target_k {
                    if a.is_empty() {
                        puts!("0");
                        continue 'o;
                    }
                    target_k = a.pop().unwrap();
                } else if !st.is_empty() && st.last().unwrap().0 == k {
                    st.last_mut().unwrap().1 += 1
                } else {
                    st.push((k, 1));
                }
            }
            let mut result = quick_pow(p, target_k, MOD);
            result -= st
                .iter()
                .map(|&(k, c)| quick_pow(p, k, MOD) * c)
                .sum::<i64>();
            puts!("{}", (result % MOD + MOD) % MOD);
        }
    }
}

mod num1 {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        for _ in 0..t {
            let len: usize = scanner.next();
            let p: i64 = scanner.next();
            let mut a: Vec<i64> = (0..len).map(|_| scanner.next()).collect();
            a.sort_unstable();
            a.reverse();
            let mut sign = vec![0; len];
            let mut sum = 0;
            for i in 0..len {
                if sum <= 0 {
                    sum += 1;
                    sign[i] = 1;
                } else {
                    sum -= 1;
                    sign[i] = -1;
                }
                if i != len - 1 {
                    let d = a[i] - a[i + 1];
                    if sum > 0 && p > 1 {
                        for _ in 0..d.min(32) {
                            sum *= p;
                            if sum >= i32::MAX as i64 {
                                sum = 2 * i32::MAX as i64;
                                break;
                            }
                        }
                    }
                }
            }
            a.push(0);
            let mut ans = 0;
            for i in 0..len {
                ans = (ans + sign[i] + MOD) % MOD;
                ans = (ans * quick_pow(p, a[i] - a[i + 1], MOD)) % MOD;
            }
            puts!("{}", ans);
        }
    }
}

fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    num1::solve(Scanner::new(stdin), &mut stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "4\n5 2\n2 3 4 4 3\n3 1\n2 10 1000\n4 5\n0 1 1 100\n1 8\n89\n",
            "4\n1\n146981438\n747093407\n",
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
            num1::solve,
            my::solve,
        ];
        for func in functions {
            for tc in testcases.chunks(2) {
                let mut output = Vec::new();
                func(Scanner::new(tc[0].as_bytes()), &mut output);
                assert_eq!(String::from_utf8(output).unwrap(), tc[1], "input: {}", tc[0]);
            }
        }
    }
}
