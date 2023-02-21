//! 【XR-4】歌唱比赛
//! https://www.luogu.com.cn/record/list?status=&pid=P5595&page=1&orderBy=1

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let s: String = scanner.next();
        let s = s.into_bytes();
        let len = s.len();
        let mut z = false;
        for &ch in &s {
            if z && ch != b'Z' {
                puts!("-1");
                return;
            } else if ch == b'Z' {
                z = true;
            }
        }
        let mut x = vec![0; len];
        let mut y = vec![0; len];
        for i in 0..len {
            if s[i] == b'X' {
                x[i] = b'1';
                y[i] = b'0'
            } else if s[i] == b'Y' {
                x[i] = b'0';
                y[i] = b'1';
            } else {
                x[i] = b'0';
                y[i] = b'0';
            }
        }
        puts!("{}", unsafe {String::from_utf8_unchecked(x)});
        puts!("{}", unsafe {String::from_utf8_unchecked(y)});
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
            "XY\n",
            "10\n01\n",
            "XYZ\n",
            "100\n010\n",
            "ZZZZZZ\n",
            "000000\n000000\n",
            "XYZXYZ\n",
            "-1\n",
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
