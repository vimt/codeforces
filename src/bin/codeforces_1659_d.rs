//! D. Reverse Sort Sum
//! https://codeforces.com/problemset/status/1659/problem/D
//! https://www.luogu.com.cn/problem/solution/CF1659D

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    /// 思路：最后一行是有序的，如果 a[-1] == n，说明最后一个位置是1。
    /// 把最后一行的1从c减掉，继续判断n-1。去掉的大小用差分数组/线段树/树状数组
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let mut sum = 0;
            let mut a: Vec<usize> = vec![0; len];
            for i in 0..len {
                a[i] = scanner.next();
                sum += a[i];
            }
            let mut one = sum / len;
            let mut sub = vec![0; len];
            let mut cur = 0;
            for i in (0..len).rev() {
                cur -= sub[i];
                if i >= one {
                    sub[i - one] += 1;
                }
                if a[i] == cur + i + 1 {
                    a[i] = 1;
                    one -= 1;
                } else {
                    a[i] = 0;
                }
                cur += 1;
            }

            for num in a {
                let _ = write!(out, "{} ", num);
            }
            let _ = writeln!(out);
        };
        for _ in 0..t {
            go();
        }
    }
}

mod other {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let mut a: Vec<usize> = vec![1; len];
            for i in 0..len {
                let c: usize = scanner.next();
                if c == 0 { a[i] = 0; }
                if a[i] == 0 { if i + c < len { a[i + c] = 0; } } else { if c < len { a[c] = 0; } }
            }
            for num in a {
                let _ = write!(out, "{} ", num);
            }
            let _ = writeln!(out);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "1\n1\n1",
            "1 \n",
            "5\n4\n2 4 2 4\n7\n0 3 4 2 3 2 7\n3\n0 0 0\n4\n0 0 0 4\n3\n1 2 3",
            "1 1 0 1 \n0 1 1 0 0 0 1 \n0 0 0 \n0 0 0 1 \n1 0 1 \n",
        ];
        let functions: Vec<fn(&mut Scanner<_>, &mut _)> = vec![
            my::solve,
            other::solve,
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
