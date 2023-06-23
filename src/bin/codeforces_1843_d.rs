//! D. Apple Tree
//! https://codeforces.com/problemset/status/1843/problem/D?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1843D

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let mut g = vec![vec![]; len];
            for _ in 0..len - 1 {
                let (a, b): (usize, usize) = (scanner.next(), scanner.next());
                g[a - 1].push(b - 1);
                g[b - 1].push(a - 1);
            }
            fn dfs(g: &Vec<Vec<usize>>, u: usize, fa: usize, sub: &mut Vec<i64>) -> i64 {
                if g[u].len() == 1 && u != 0 {
                    sub[u] = 1;
                    return 1;
                }
                let mut result = 0;
                for &v in &g[u] {
                    if v != fa {
                        result += dfs(g, v, u, sub);
                    }
                }
                sub[u] = result;
                result
            }
            let mut sub = vec![0; len];
            dfs(&g, 0, len, &mut sub);

            let q: usize = scanner.next();
            for _ in 0..q {
                let (a, b): (usize, usize) = (scanner.next(), scanner.next());
                puts!("{}", sub[a-1] * sub[b-1]);
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

#[cfg(debug)]
fn main() {
    use codeforces::solves;
    use codeforces::tester::Tester;
    let t = Tester::new(solves!(my));
    t.test("2\n5\n1 2\n3 4\n5 3\n3 2\n4\n3 4\n5 1\n4 4\n1 3\n3\n1 2\n1 3\n3\n1 1\n2 3\n3 1\n2\n5\n5 1\n1 2\n2 3\n4 3\n2\n5 5\n5 1\n5\n3 2\n5 3\n2 1\n4 2\n3\n4 3\n2 1\n4 2",
           "2\n2\n1\n4\n4\n1\n2\n");
}
