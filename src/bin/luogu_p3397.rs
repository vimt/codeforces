//! 地毯
//! https://www.luogu.com.cn/record/list?status=&pid=P3397&page=1&orderBy=1

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        let n: usize = scanner.next();
        let m: usize = scanner.next();
        let mut a = vec![vec![0; n + 2]; n + 2];
        for _ in 0..m {
            let x1: usize = scanner.next();
            let y1: usize = scanner.next();
            let x2: usize = scanner.next();
            let y2: usize = scanner.next();
            a[x1][y1] += 1;
            a[x2 + 1][y1] -= 1;
            a[x1][y2 + 1] -= 1;
            a[x2 + 1][y2 + 1] += 1;
        }
        for i in 1..=n {
            for j in 1..=n {
                a[i][j] += a[i - 1][j] + a[i][j - 1] - a[i - 1][j - 1];
            }
        }
        for i in 1..=n {
            for j in 1..=n {
                let _ = write!(out, "{}{}", a[i][j], if j == n { "\n" } else { " " });
            }
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
            "5 3\n2 2 3 3\n3 3 5 5\n1 2 1 4",
            "0 1 1 1 0\n0 1 1 0 0\n0 1 2 1 1\n0 0 1 1 1\n0 0 1 1 1\n",
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
