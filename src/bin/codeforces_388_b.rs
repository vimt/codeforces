//! B. Fox and Minimal path
//! https://codeforces.com/problemset/status/388/problem/B
//! https://www.luogu.com.cn/problem/solution/CF388B

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    /// 考虑k的二进制：先造 2^n 次方的方案，然后考虑旁边有一个竖列搭着，搭到2^n树上，对应的位数就是1
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let k: usize = scanner.next();
        if k == 1 {
            puts!("2");
            puts!("NY");
            puts!("YN");
            return;
        }
        let level = 63 - k.leading_zeros() as usize;
        let n = level * 3 + 2;
        let ps = level * 2 + 2;
        let mut result = vec![vec![b'N'; n]; n];
        let mut conn = |a: usize, b: usize| {
            result[a][b] = b'Y';
            result[b][a] = b'Y';
        };
        conn(0, 2);
        conn(0, 3);
        conn(level * 2, 1);
        conn(level * 2 + 1, 1);
        for l in 1..level {
            let (a, b) = (l * 2, l * 2 + 1);
            let (c, d) = (l * 2 + 2, l * 2 + 3);
            conn(a, c);
            conn(a, d);
            conn(b, c);
            conn(b, d);
        }
        conn(0, ps + level - 1);
        for i in ps..ps + level - 1 {
            conn(i, i + 1);
        }
        if k & 1 == 1 {
            conn(ps, 1);
        }
        for i in 1..level {
            if k >> i & 1 == 1 {
                conn(ps + i, (level - i + 1) * 2);
                conn(ps + i, (level - i + 1) * 2 + 1);
            }
        }

        puts!("{}", n);
        for line in result {
            puts!("{}", unsafe {String::from_utf8_unchecked(line)});
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
            "4\n",
            "",
            "2\n",
            "4\nNNYY\nNNYY\nYYNN\nYYNN\n",
            "9\n",
            "8\nNNYYYNNN\nNNNNNYYY\nYNNNNYYY\nYNNNNYYY\nYNNNNYYY\nNYYYYNNN\nNYYYYNNN\nNYYYYNNN\n",
            "1\n",
            "2\nNY\nYN\n",
        ];
        let functions: Vec<fn(&mut Scanner<_>, &mut _)> = vec![
            my::solve,
        ];
        for func in functions {
            for tc in testcases.chunks(2) {
                let mut output = Vec::new();
                func(&mut Scanner::new(tc[0].as_bytes()), &mut output);
                let mut sc = Scanner::new(tc[0].as_bytes());
                let k: usize = sc.next();
                sc = Scanner::new(&output);
                let n: usize = sc.next();
                let mut grid = vec![];
                for _ in 0..n {
                    grid.push(sc.next::<String>().into_bytes());
                }
                let mut q = vec![(0, 0)];
                while !q.is_empty() {
                    let mut nq = vec![];
                    for (u, fa) in q {
                        for v in 0..n {
                            if v != fa && grid[u][v] == b'Y' {
                                nq.push((v, u));
                            }
                        }
                    }
                    if nq.iter().any(|x| x.0 == 1) {
                        assert_eq!(nq.iter().filter(|x| x.0 == 1).count(), k);
                        break;
                    }
                    q = nq;
                }
            }
        }
    }
}
