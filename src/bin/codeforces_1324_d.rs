//! https://codeforces.com/contest/1324/problem/D
//! D. Pair of Topics

use codeforces::scanner::Scanner;

mod my {
    use std::cmp::Ordering;
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let len: usize = scanner.next();
        let mut a: Vec<i64> = (0..len).map(|_| scanner.next()).collect();
        for i in 0..len {
            a[i] -= scanner.next::<i64>();
        }
        a.sort_unstable();
        let mut result = 0;
        for i in 0..len - 1 {
            if a[i] <= 0 {
                let j = a[i + 1..].binary_search_by(|x| x.cmp(&(-a[i])).then(Ordering::Less)).unwrap_err();
                result += len - 1 - j - i;
            } else {
                result += len - i - 1;
            }
        }
        puts!("{}", result);
    }
}

mod double_point {
    use std::io::{BufRead, Write};
    use super::*;

    /// 自己卡了半天没写出来的双指针，原来是这么写的
    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let len: usize = scanner.next();
        let mut a: Vec<i64> = (0..len).map(|_| scanner.next()).collect();
        for i in 0..len {
            a[i] -= scanner.next::<i64>();
        }
        a.sort_unstable();
        let mut result = 0;
        let mut l = 0;
        let mut r = len;
        while l < r {
            if a[l] + a[r - 1] > 0 {
                r -= 1;
                result += r - l;
            } else {
                l += 1;
            }
        }
        puts!("{}", result);
    }
}


fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    double_point::solve(Scanner::new(stdin), &mut stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "5\n4 8 2 6 2\n4 5 4 1 3\n",
            "7\n",
            "4\n1 3 2 4\n1 3 2 4\n",
            "0\n",
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
            double_point::solve,
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
