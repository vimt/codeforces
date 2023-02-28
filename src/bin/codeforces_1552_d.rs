//! D. Array Differentiation
//! https://codeforces.com/problemset/status/1552/problem/D
//! https://www.luogu.com.cn/problem/solution/CF1552D

use codeforces::scanner::Scanner;

mod my {
    use std::collections::HashSet;
    use std::io::{BufRead, Write};
    use super::*;

    /// b为a[..n-1]的前缀和，那么如果存在 a[n-1] = b[j] - b[i] = a[i+1] + .. + a[j]，就存在b
    /// 判断 a 中是否存在两个子集，其子集和相等。
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let a: Vec<i32> = (0..len).map(|_| scanner.next()).collect();
            let mut set = HashSet::with_capacity(1 << len);
            for i in 0..1 << len {
                let mut sum = 0;
                for (j, &num) in a.iter().enumerate() {
                    if i >> j & 1 == 1 {
                        sum += num;
                    }
                }
                set.insert(sum);
            }
            if set.len() < 1 << len {
                puts!("YES");
            } else {
                puts!("NO");
            }
        };
        for _ in 0..t {
            go();
        }
    }
}

#[cfg(not(debug))]
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
            "5\n5\n4 -7 -1 5 10\n1\n0\n3\n1 10 100\n4\n-3 2 10 2\n9\n25 -171 250 174 152 242 100 -205 -258\n",
            "YES\nYES\nNO\nYES\nYES\n",
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
