//! B. Johnny and Grandmaster

use codeforces::scanner::Scanner;

const MOD: i64 = 1e9 as i64 + 7;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

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
            let mut target_k = a[len - 1];
            let mut s: Vec<(i64, i64)> = vec![];
            let mut i = len - 1;
            while i > 0 {
                let mut k = a[i-1];
                while !s.is_empty() && *s.last().unwrap() == (k, p - 1) {
                    s.pop().unwrap();
                    k += 1;
                }
                if k == target_k {
                    if i == 1 {
                        puts!("{}", 0);
                        continue 'o;
                    }
                    i -= 1;
                    target_k = a[i-1];
                } else if !s.is_empty() && s.last().unwrap().0 == k {
                    s.last_mut().unwrap().1 += 1;
                } else {
                    s.push((k, 1));
                }
                i -= 1;
            }
            let mut result = quick_pow(p, target_k, MOD);
            for (k, c) in s {
                result -= quick_pow(p, k, MOD) * c;
            }
            puts!("{}", (result % MOD + MOD) % MOD);
        }
    }
}

fn main() {
    use std::io::{stdin, stdout};
    my::solve(Scanner::new(stdin().lock()), &mut stdout().lock())
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
            my::solve
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
