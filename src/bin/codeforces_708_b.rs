//! B. Recover the String
//! https://codeforces.com/problemset/status/708/problem/B
//! https://www.luogu.com.cn/problem/solution/CF708B

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let x00: i64 = scanner.next();
        let x01: i64 = scanner.next();
        let x10: i64 = scanner.next();
        let x11: i64 = scanner.next();
        let test = |x0, x1| {
            x0 * (x0 - 1) / 2 == x00 && x1 * (x1 - 1) / 2 == x11 && x0 * x1 == (x01 + x10)
        };

        let mut possible_x0 = vec![(1 + ((1 + 8 * x00) as f64).sqrt() as i64) / 2];
        let mut possible_x1 = vec![(1 + ((1 + 8 * x11) as f64).sqrt() as i64) / 2];
        if x00 == 0 { possible_x0.push(0); }
        if x11 == 0 { possible_x1.push(0); }
        for x0 in possible_x0 {
            for &x1 in &possible_x1 {
                if test(x0, x1) {
                    let len = (x0 + x1) as usize;
                    if len == 0 {
                        puts!("0");
                        return;
                    }
                    let mut result = vec![0; len];
                    let mut c0 = 0;
                    let mut c1 = 0;
                    let mut c01 = 0;
                    let mut c10 = 0;
                    for i in 0..len {
                        if c0 < x0 {
                            // 选0
                            c0 += 1;
                            c10 += c1;
                            // 如果选了0，最大可能的01数量<c01 || 最大可能10的数量<c10，就选错了
                            let mc = (x1 - c1) * (x0 - c0);
                            if c0 * (x1 - c1) + mc < (x01 - c01) || c1 * (x0 - c0) + mc < (x10 - c10) {
                                c0 -= 1;
                                c10 -= c1;
                                c1 += 1;
                                c01 += c0;
                                result[i] = b'1';
                            } else {
                                result[i] = b'0';
                            }
                        } else {
                            c1 += 1;
                            c01 += c0;
                            result[i] = b'1';
                        }
                    }
                    puts!("{}", unsafe{String::from_utf8_unchecked(result)});
                    return;
                }
            }
        }
        puts!("Impossible");
    }
}

mod other {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let x00: i64 = scanner.next();
        let mut x01: i64 = scanner.next();
        let x10: i64 = scanner.next();
        let x11: i64 = scanner.next();
        let x0 = if x00 != 0 { (1 + ((1 + 8 * x00) as f64).sqrt() as i64) / 2 } else { (x01 > 0 || x10 > 0) as i64 };
        let mut x1 = if x11 != 0 { (1 + ((1 + 8 * x11) as f64).sqrt() as i64) / 2 } else { (x01 > 0 || x10 > 0) as i64 };
        if x0 * (x0 - 1) / 2 != x00 || x1 * (x1 - 1) / 2 != x11 || x0 * x1 != (x01 + x10) {
            puts!("Impossible");
            return;
        }
        let len = (x0 + x1) as usize;
        let mut result = vec![0; len];
        for i in 0..len {
            // 选0，需要后面1
            if x01 >= x1 {
                x01 -= x1;
                result[i] = b'0';
            } else {
                x1 -= 1;
                result[i] = b'1';
            }
        }
        if result.is_empty() { result.push(b'0'); }
        puts!("{}", unsafe{String::from_utf8_unchecked(result)});
    }
}

fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    other::solve(&mut Scanner::new(stdin), &mut stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "3 1 2 0\n",
            "0100\n",
            "1 0 0 0\n",
            "00\n",
            "0 0 0 0\n",
            "0\n",
            "1 2 3 4\n",
            "Impossible\n",
            "1 2 2 1\n",
            "0110\n",
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
