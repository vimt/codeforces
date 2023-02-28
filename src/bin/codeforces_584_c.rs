//! C. Marina and Vasya
//! https://codeforces.com/problemset/status/584/problem/C
//! https://www.luogu.com.cn/problem/solution/CF584C

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    /// a[i]==b[i]的时候，一个字符能满足1一个相同，a[i]!=b[i]的时候，两个字符才能满足一个相同
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let len: usize = scanner.next();
        let t = len - scanner.next::<usize>();
        let mut s1 = scanner.next::<String>().into_bytes();
        let s2 = scanner.next::<String>().into_bytes();
        let mut same = 0;
        for (a, b) in s1.iter().zip(&s2) {
            if a == b {
                same += 1;
            }
        }
        if t > same && len - same < (t - same) * 2 {
            puts!("-1");
            return;
        }
        let mut t = t as i32;
        let mut a = t - same as i32;
        let mut b = t - same as i32;
        fn another(a: u8, b: u8) -> u8 {
            for x in b'a'..=b'z' {
                if x != a && x != b {
                    return x;
                }
            }
            unreachable!()
        }
        for (c1,c2) in s1.iter_mut().zip(s2) {
            if *c1 == c2 {
                if t <= 0 { *c1 = another(*c1, c2); } else { t -= 1; }
            } else {
                if a > 0 {
                    a -= 1;
                } else if b > 0 {
                    *c1 = c2;
                    b -= 1;
                } else {
                    *c1 = another(*c1, c2);
                }
            }
        }
        puts!("{}", unsafe{String::from_utf8_unchecked(s1)});
    }
}

fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    my::solve(&mut Scanner::new(stdin), &mut stdout)
}

#[cfg(test)]
mod tests {
    use codeforces::input;
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "3 2\nabc\nxyc\n",
            "ayd\n",
            "1 0\nc\nb\n",
            "-1\n",
        ];
        let functions: Vec<fn(&mut Scanner<_>, &mut _)> = vec![
            my::solve,
        ];
        for func in functions {
            for tc in testcases.chunks(2) {
                let mut output = Vec::new();
                macro_rules! aeq {
                    ($left:expr, $right:expr) => {assert_eq!($left, $right, "input:\n{}\nexpect:\n{}\noutput:\n{}", tc[0],tc[1],std::str::from_utf8(&output).unwrap())};
                }
                func(&mut Scanner::new(tc[0].as_bytes()), &mut output);
                let mut sc = Scanner::new(tc[0].as_bytes());
                input! {sc, _n:usize, t:usize,a:String,b:String}
                if tc[1] == "-1\n" {
                    aeq!(b"-1", &output[..2]);
                } else {
                    let t1 = a.as_bytes().iter().zip(&output).filter(|x| x.0 != x.1).count();
                    let t2 = b.as_bytes().iter().zip(&output).filter(|x| x.0 != x.1).count();
                    aeq!(t, t1);
                    aeq!(t, t2);
                }
            }
        }
    }
}
