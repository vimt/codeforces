//! C. Make Palindrome
//! https://codeforces.com/problemset/status/600/problem/C
//! https://www.luogu.com.cn/problem/solution/CF600C

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let mut s = scanner.next::<String>().into_bytes();
        let len = s.len();
        let mut cnt = [0; 26];
        for &ch in &s {
            cnt[(ch - b'a') as usize] += 1;
        }
        let mut i = 0;
        let mut j = 25;
        while i < j {
            while i < j && cnt[i] & 1 == 0 { i += 1; }
            while i < j && cnt[j] & 1 == 0 { j -= 1; }
            if i == j { break; }
            cnt[i] += 1;
            cnt[j] -= 1;
        }
        let mut offset = 0;
        for i in 0..26 {
            let ch = i as u8 + b'a';
            for _ in 0..cnt[i] / 2 {
                s[offset] = ch;
                s[len - 1 - offset] = ch;
                offset += 1;
            }
            if len & 1 == 1 && cnt[i] & 1 == 1 {
                s[len / 2] = ch;
            }
        }
        puts!("{}", unsafe {String::from_utf8_unchecked(s)});
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
            "abcd\n",
            "abba\n",
            "aabc\n",
            "abba\n",
            "aabcd\n",
            "abcba\n",
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
