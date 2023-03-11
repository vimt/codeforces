//! D. Choosing Capital for Treeland
//! https://codeforces.com/problemset/status/219/problem/D
//! https://www.luogu.com.cn/problem/solution/CF219D
//! 给一个有向图，f(x)=以x为根，需要反边的数量，输出min(f(x))和所有对应x

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use codeforces::graph::Graph;
    use super::*;


    /// 换根DP
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let n: usize = scanner.next();
        let mut g: Graph<i32> = Graph::new(n + 1, n * 2);
        for _ in 0..n - 1 {
            let a: usize = scanner.next();
            let b: usize = scanner.next();
            g.add(a, b, 0);
            g.add(b, a, 1);
        }
        // 以u为根的反边数量
        fn dfs(g: &Graph<i32>, u: usize, fa: usize) -> i32 {
            let mut result = 0;
            for (v, &x) in g.neigh(u) {
                if v != fa {
                    result += x + dfs(g, v, u);
                }
            }
            result
        }
        // a表示以u为根的反边数量
        fn reroot(g: &Graph<i32>, u: usize, fa: usize, a: i32, dp: &mut Vec<i32>) {
            dp[u] = a;
            for (v, &x) in g.neigh(u) {
                if v != fa {
                    reroot(g, v, u, a + if x == 0 { 1 } else { -1 }, dp);
                }
            }
        }
        let a = dfs(&g, 1, 0);
        let mut dp = vec![0; n + 1];
        reroot(&g, 1, 0, a, &mut dp);
        let min = *dp[1..].iter().min().unwrap();
        puts!("{}", min);
        for i in 1..=n {
            if dp[i] == min {
                let _ = write!(out, "{} ", i);
            }
        }
        puts!("");
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
    t.test("3\n2 1\n2 3\n",
           "0\n2 \n");
    t.test("4\n1 4\n2 4\n3 4\n",
           "2\n1 2 3 \n");
}
