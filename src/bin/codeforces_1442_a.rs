//! A. Extreme Subtraction

use std::io::{stdin, stdout};
use std::io::prelude::*;
use codefoces::scanner::Scanner;

fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
    macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out, $($format)*););}
    let t: usize = scanner.next();
    for _ in 0..t {
        let len: usize = scanner.next();
        let mut mx = i32::MAX;
        let mut mn = 0;
        let mut ok = true;
        for _ in 0..len {
            let num: i32 = scanner.next();
            if mn > num {
                ok = false;
            } else {
                let if_all_mx = num - mn;
                if if_all_mx <= mx {
                    mx = if_all_mx;
                } else {
                    let other = if_all_mx - mx;
                    mn += other;
                }
            }
        }
        puts!("{}", if ok { "YES" } else { "NO" });
    }
}

fn main() {
    solve(Scanner::new(stdin().lock()), &mut stdout().lock())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "4\n3\n1 2 1\n5\n11 7 9 6 8\n5\n1 3 1 3 1\n4\n5 2 1 10\n",
            "YES\nYES\nNO\nYES\n",
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
            solve
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
