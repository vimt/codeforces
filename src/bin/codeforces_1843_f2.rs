//! F2. Omsk Metro (hard version)
//! https://codeforces.com/problemset/status/1843/problem/F2?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1843F2

use codeforces::scanner::Scanner;

mod dalao {
    use std::io::{BufRead, Write};
    use std::mem::swap;
    use std::ops::Add;
    use super::*;

    const K: usize = 18;

    #[derive(Debug, Copy, Clone, Default)]
    struct Info {
        sum: i32,
        maxpre: i32,
        maxsuf: i32,
        maxsegsum: i32,
        minpre: i32,
        minsuf: i32,
        minsegsum: i32,
    }

    impl Add for Info {
        type Output = Info;

        fn add(self, rhs: Self) -> Self::Output {
            return Info {
                sum: self.sum + rhs.sum,
                maxpre: self.maxpre.max(self.sum + rhs.maxpre),
                maxsuf: rhs.maxsuf.max(self.maxsuf + rhs.sum),
                maxsegsum: self.maxsegsum.max(rhs.maxsegsum).max(self.maxsuf + rhs.maxpre),
                minpre: self.minpre.min(self.sum + rhs.minpre),
                minsuf: rhs.minsuf.min(self.minsuf + rhs.sum),
                minsegsum: self.minsegsum.min(rhs.minsegsum).min(self.minsuf + rhs.minpre),
            };
        }
    }

    impl Info {
        fn new(x: i32) -> Info {
            return Info {
                sum: x,
                maxpre: x.max(0),
                maxsuf: x.max(0),
                maxsegsum: x.max(0),
                minpre: x.min(0),
                minsuf: x.min(0),
                minsegsum: x.min(0),
            };
        }
        fn rev(self) -> Info {
            return Info {
                sum: self.sum,
                maxpre: self.maxsuf,
                maxsuf: self.maxpre,
                maxsegsum: self.maxsegsum,
                minpre: self.minsuf,
                minsuf: self.minpre,
                minsegsum: self.minsegsum,
            };
        }
    }

    /// 树上倍增+合并子段
    /// https://codeforces.com/problemset/submission/1843/210486795
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let mut nodes = vec![1];
            let mut depth = vec![0];
            let mut parent = vec![[0; K]];
            let mut lifts = vec![[Info::default(); K]];
            nodes.reserve(len);
            depth.reserve(len);
            parent.reserve(len);
            lifts.reserve(len);
            for _ in 0..len {
                let t: String = scanner.next();
                if t == "+" {
                    let v: usize = scanner.next::<usize>() - 1;
                    let x: i32 = scanner.next();
                    parent.push([v; K]);
                    nodes.push(x);
                    depth.push(depth[v] + 1);
                    lifts.push([Info::new(x); K]);
                    for i in 1..K {
                        parent.last_mut().unwrap()[i] = parent[parent.last().unwrap()[i - 1]][i - 1];
                        lifts.last_mut().unwrap()[i] = lifts[parent.last().unwrap()[i - 1]][i - 1] + lifts.last().unwrap()[i - 1]
                    }
                } else if t == "?" {
                    let mut u = scanner.next::<usize>() - 1;
                    let mut v = scanner.next::<usize>() - 1;
                    let k: i32 = scanner.next();
                    let mut l = Info::default();
                    let mut r = Info::default();

                    if depth[u] > depth[v] { swap(&mut u, &mut v); }
                    let diff = depth[v] - depth[u];
                    for i in 0..K {
                        if diff >> i & 1 == 1 {
                            r = lifts[v][i] + r;
                            v = parent[v][i];
                        }
                    }

                    if u != v {
                        for i in (0..K).rev() {
                            if parent[u][i] != parent[v][i] {
                                l = lifts[u][i] + l;
                                r = lifts[v][i] + r;
                                u = parent[u][i];
                                v = parent[v][i];
                            }
                        }
                        l = lifts[u][0] + l;
                        r = lifts[v][0] + r;
                        u = parent[u][0];
                    }

                    let tot = l.rev() + Info::new(nodes[u]) + r;
                    if k >= tot.minsegsum && k <= tot.maxsegsum { puts!("YES"); } else { puts!("NO"); }
                } else {
                    unreachable!()
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
    dalao::solve(&mut Scanner::new(stdin), &mut stdout)
}

#[cfg(debug)]
fn main() {
    use codeforces::solves;
    use codeforces::tester::Tester;
    let t = Tester::new(solves!(dalao));
    t.test("1\n8\n+ 1 -1\n? 1 1 2\n? 1 2 1\n+ 1 1\n? 1 3 -1\n? 1 1 1\n? 1 3 2\n? 1 1 0\n1\n7\n+ 1 -1\n+ 2 -1\n+ 2 1\n+ 3 -1\n? 5 2 2\n? 3 1 -1\n? 5 4 -3",
           "NO\nYES\nNO\nYES\nYES\nYES\n");
}
