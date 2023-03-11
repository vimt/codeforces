//! D. Book of Evil
//! https://codeforces.com/problemset/status/337/problem/D?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF337D
//! 一个树上有一些evil节点，统计距离所有evil节点距离<=d的节点数

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use codeforces::graph::Graph;
    use super::*;

    /// 换根DP
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let n: usize = scanner.next();
        let m: usize = scanner.next();
        let d: i32 = scanner.next();
        let mut evil = vec![false; n + 1];
        for _ in 0..m {
            evil[scanner.next::<usize>()] = true;
        }
        let mut g = Graph::new(n + 1, n * 2);
        for _ in 0..n - 1 {
            let u: usize = scanner.next();
            let v: usize = scanner.next();
            g.add(u, v, ());
            g.add(v, u, ());
        }

        fn dfs(g: &Graph, evil: &Vec<bool>, u: usize, fa: usize, dp: &mut Vec<(i32, i32)>) {
            if evil[u] { dp[u] = (0, 0); }
            for (v, _) in g.neigh(u) {
                if v != fa {
                    dfs(g, evil, v, u, dp);
                    if  dp[v].0 + 1 > dp[u].0 {
                        dp[u].1 = dp[u].0;
                        dp[u].0 = dp[v].0 + 1;
                    } else if  dp[v].0 + 1 > dp[u].1 {
                        dp[u].1 = dp[v].0 + 1;
                    }
                }
            }
        }
        let mut dp = vec![(i32::MIN, i32::MIN); n + 1]; // (子树怪物最远距离，子树怪物次远距离）
        struct ReRoot<'a> {
            d: i32,
            g: &'a Graph,
            dp: Vec<(i32, i32)>,
            result: i32,
        }

        impl<'a> ReRoot<'a> {
            fn dfs(&mut self, u: usize, fa: usize, ff: i32) {
                if ff <= self.d && self.dp[u].0 <= self.d {
                    self.result += 1;
                }
                for (v, _) in self.g.neigh(u) {
                    if v != fa {
                        let nff = if self.dp[u].0 == self.dp[v].0 + 1 { // v是u的最远怪物分支
                            ff.max(self.dp[u].1) + 1
                        } else {
                            ff.max(self.dp[u].0) + 1
                        };
                        self.dfs(v, u, nff);
                    }
                }
            }
        }
        dfs(&g, &evil, 1, 0, &mut dp);
        let mut rr = ReRoot { d, g: &g, dp, result: 0 };
        rr.dfs(1, 0, i32::MIN);
        puts!("{}", rr.result);
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
    t.test("6 2 3\n1 2\n1 5\n2 3\n3 4\n4 5\n5 6\n",
           "3\n");
    t.test("2 2 1\n2 1\n1 2",
           "2\n");
}
