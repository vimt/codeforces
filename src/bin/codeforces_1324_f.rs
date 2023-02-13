//! https://codeforces.com/contest/1324/problem/F
//! F. Maximum White Subtree

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        let len: usize = scanner.next();
        let mut a: Vec<i32> = vec![0; len + 1];
        for i in 1..=len {
            a[i] = if scanner.next::<i8>() == 1 { 1 } else { -1 };
        }
        // 使用数组链表的方式避免数组扩容（例如 g = vec![vec![]; len] ）
        let mut head = vec![0; len * 2 + 5];
        let mut e = vec![(0, 0); len * 2 + 5];
        let mut cnt = 0;
        let mut add = |u, v| {
            cnt += 1;
            e[cnt] = (head[u], v);
            head[u] = cnt;
        };
        for _ in 0..len - 1 {
            let u: usize = scanner.next();
            let v: usize = scanner.next();
            add(u, v);
            add(v, u);
        }
        fn dfs1(a: &mut Vec<i32>, head: &Vec<usize>, e: &Vec<(usize, usize)>, u: usize, fa: usize) {
            let mut i = head[u];
            while i > 0 {
                let v = e[i].1;
                if v != fa {
                    dfs1(a, head, e, v, u);
                    if a[v] > 0 { a[u] += a[v]; }
                }
                i = e[i].0;
            }
        }
        fn dfs2(a: &mut Vec<i32>, head: &Vec<usize>, e: &Vec<(usize, usize)>, u: usize, fa: usize) {
            let mut i = head[u];
            while i > 0 {
                let v = e[i].1;
                if v != fa {
                    if a[v] >= 0 {
                        a[v] = a[v].max(a[u]);
                    } else {
                        a[v] = a[v].max(a[u] + a[v]);
                    }
                    dfs2(a, head, e, v, u);
                }
                i = e[i].0;
            }
        }

        dfs1(&mut a, &head, &e, 1, 0);
        dfs2(&mut a, &head, &e, 1, 0);
        for i in 1..=len {
            write!(out, "{} ", a[i]).ok();
        }
        writeln!(out).ok();
    }
}

fn main() {
    use codeforces::raw;
    let (stdin, mut stdout) = raw::in_out();
    my::solve(Scanner::new(stdin), &mut stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "9\n0 1 1 1 0 0 0 0 1\n1 2\n1 3\n3 4\n3 5\n2 6\n4 7\n6 8\n5 9\n",
            "2 2 2 2 2 1 1 0 2 \n",
            "4\n0 0 1 0\n1 2\n1 3\n1 4\n",
            "0 -1 1 -1 \n",
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
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
