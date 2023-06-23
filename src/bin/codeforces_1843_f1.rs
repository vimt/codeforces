//! F1. Omsk Metro (simple version)
//! https://codeforces.com/problemset/status/1843/problem/F1?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1843F1

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let mut nodes = vec![1];
            let mut g = vec![vec![]];
            let mut q: Vec<(usize, usize, i32)> = vec![];
            for _ in 0..len {
                let t: String = scanner.next();
                if t == "+" {
                    let u: usize = scanner.next();
                    let w: i32 = scanner.next();
                    nodes.push(w);
                    g.push(vec![]);
                    g[u - 1].push(nodes.len() - 1);
                } else if t == "?" {
                    q.push((scanner.next(), scanner.next(), scanner.next()));
                }
            }

            // mn: 以u结尾的0-u 的最小子段和，pmn: 0-u的最小的 mn
            fn dfs(node: &Vec<i32>, g: &Vec<Vec<usize>>, u: usize, fa: usize, mut mn: i32, mut mx: i32, pmn: &mut Vec<i32>, pmx: &mut Vec<i32>) {
                mn = (mn + node[u]).min(0);
                mx = (mx + node[u]).max(0);
                pmn[u] = pmn[fa].min(mn);
                pmx[u] = pmx[fa].max(mx);
                for &v in &g[u] {
                    if v != fa {
                        dfs(node, g, v, u, mn, mx, pmn, pmx);
                    }
                }
            }

            let mut mn = vec![0; nodes.len() + 1];
            let mut mx = vec![0; nodes.len() + 1];
            dfs(&nodes, &g, 0, nodes.len(), 0, 0, &mut mn, &mut mx);
            for (_, v, k) in q {
                if k >= mn[v - 1] && k <= mx[v - 1] {
                    puts!("YES");
                } else {
                    puts!("NO");
                }
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
    t.test("1\n8\n+ 1 -1\n? 1 1 2\n? 1 2 1\n+ 1 1\n? 1 3 -1\n? 1 1 1\n? 1 3 2\n? 1 1 0\n1\n10\n+ 1 -1\n+ 1 -1\n+ 3 1\n+ 3 -1\n+ 3 1\n? 1 6 -1\n? 1 6 2\n? 1 2 0\n? 1 5 -2\n? 1 4 3",
           "NO\nYES\nNO\nYES\nYES\nYES\n");
}
