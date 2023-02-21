//! https://codeforces.com/contest/1213/problem/D2
//! D2. Equalizing by Division (hard version)

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let n: usize = scanner.next();
        let k: i32 = scanner.next();
        const MAX: usize = 200001;
        let mut a = vec![Vec::with_capacity(16); MAX];
        for _ in 0..n {
            let i = scanner.next::<usize>();
            if a[i].is_empty() { a[i].push(0); }
            a[i][0] += 1;
        }
        let mut result = i32::MAX;
        for i in (1..MAX).rev() {
            if a[i].is_empty() { continue; }
            let mut left = k;
            let mut times = 0;
            for (j, &c) in a[i].iter().enumerate() {
                if left <= c {
                    result = result.min(times + left * j as i32);
                    break;
                }
                left -= c;
                times += j as i32 * c;
            }
            let i2 = i >> 1;
            while a[i2].len() < a[i].len() + 1 {
                a[i2].push(0);
            }
            for j in 0..a[i].len() {
                a[i2][j + 1] += a[i][j];
            }
        }
        puts!("{}", result);
    }
}

mod num1 {
    use std::io::{BufRead, Write};
    use super::*;

    /// 例如计算 4出现k次，代价是 4 出现的次数*0 + [8,10) 出现的次数*1 + [16,20) 出现的次数*3 + ...
    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let n: usize = scanner.next();
        let k: usize = scanner.next();
        const MAX: usize = 200001;
        let mut a = vec![0; MAX];
        for _ in 0..n {
            a[scanner.next::<usize>()] += 1;
        }
        for i in 1..MAX {
            a[i] += a[i - 1];
        }
        let mut result = usize::MAX;
        for i in 1..MAX {
            let mut left = k.saturating_sub(a[i] - a[i - 1]);  // 饱和-
            let mut local = 0;
            let mut low = i * 2;
            let mut step = 1;
            while left > 0 && low < MAX {
                let ni = (low + (1 << step)).min(MAX);
                let take = left.min(a[ni - 1] - a[low - 1]);
                left -= take;
                local += step * take;
                low <<= 1;
                step += 1;
            }
            if left == 0 {
                result = result.min(local);
            }
        }


        puts!("{}", result);
    }
}


fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    num1::solve(Scanner::new(stdin), &mut stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "5 3\n1 2 2 4 5\n",
            "1\n",
            "5 3\n1 2 3 4 5\n",
            "2\n",
            "5 3\n1 2 3 3 3\n",
            "0\n",
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
            num1::solve,
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
