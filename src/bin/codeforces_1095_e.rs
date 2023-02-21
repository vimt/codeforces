//! https://codeforces.com/contest/1095/problem/E
//! E. Almost Regular Bracket Sequence

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let len: usize = scanner.next();
        let a: String = scanner.next();
        assert_eq!(a.len(), len);
        let mut s = 0;
        let mut i1 = None;
        let mut neg = false;
        let a = a.as_bytes();
        for (i, &ch) in a.iter().enumerate() {
            if ch == b'(' {
                s += 1;
            } else {
                s -= 1;
                if s < -2 {
                    puts!("0");
                    return;
                }
                if i1.is_none() && s == -1 {
                    i1 = Some(i);
                }
                if s < 0 { neg = true; }
            }
        }
        // 如果 c 为 -2，那么要改 ')'，且只能修改从开头到第一次 c=-1 这一段的 )。
        if s == -2 {
            puts!("{}", a[..=i1.unwrap()].iter().filter(|&&x|x==b')').count());
            return;
        }
        if s != 2 || neg {
            puts!("0");
            return;
        }
        let mut result = 0;
        let mut i = len;
        // 如果 c 为 2，那么要改 '('，那么 c 在任何时候都不能小于 0。倒着遍历 s，统计 c > 1 时的 '('
        while s > 1 {
            if a[i - 1] == b'(' {
                result += 1;
                s -= 1;
            } else {
                s += 1;
            }
            i -= 1;
        }
        puts!("{}", result);
    }
}

fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    my::solve(&mut Scanner::new(stdin), &mut stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "6\n(((())\n",
            "3\n",
            "6\n()()()\n",
            "0\n",
            "1\n)\n",
            "0\n",
            "8\n)))(((((\n",
            "0\n",
        ];
        let functions: Vec<fn(&mut Scanner<_>, &mut _)> = vec![
            my::solve,
        ];
        for func in functions {
            for tc in testcases.chunks(2) {
                let mut output = Vec::new();
                func(&mut Scanner::new(tc[0].as_bytes()), &mut output);
                assert_eq!(String::from_utf8(output).unwrap(), tc[1], "input: {}", tc[0]);
            }
        }
    }
}
