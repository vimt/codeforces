//! C. Longest Regular Bracket Sequence
//! https://codeforces.com/problemset/status/5/problem/C
//! https://www.luogu.com.cn/problem/solution/CF5C

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    /// LC32
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let s = scanner.next::<String>().into_bytes();
        let len = s.len();
        let mut mx = 0;
        let mut mx_cnt = 1;
        let mut stk = Vec::with_capacity(len + 1);
        stk.push(-1);
        for (i, ch) in s.into_iter().enumerate() {
            let i = i as i32;
            if ch == b'(' {
                stk.push(i);
            } else {
                stk.pop();
                if stk.is_empty() {
                    stk.push(i);
                } else {
                    let cnt = i - *stk.last().unwrap();
                    if cnt > mx {
                        mx = cnt;
                        mx_cnt = 1;
                    } else if cnt == mx { mx_cnt += 1; }
                }
            }
        }
        puts!("{} {}", mx, mx_cnt);
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
            ")((())))(()())\n",
            "6 2\n",
            "))(\n",
            "0 1\n",
            "(()())()(())()()())())()((()(()(())()()())((()(())()(()()()()))()(())()(((()())()(()((())()(())(()))\n",
            "28 1\n",
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
