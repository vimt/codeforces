//! B - Reversible Cards

use std::io::{BufReader, stdin, stdout};
use std::io::prelude::*;
use codeforces::scanner::Scanner;


pub struct UnionFind {
    pub f: Vec<usize>,
    pub size: Vec<usize>,
    pub has_ring: Vec<bool>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            f: (0..n).collect(),
            size: vec![1; n],
            has_ring: vec![false; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        return if self.f[x] == x {
            x
        } else {
            self.f[x] = self.find(self.f[x]);
            self.f[x]
        };
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut parent = self.find(x);
        let mut son = self.find(y);
        if parent == son {
            self.has_ring[parent] = true;
            return false;
        }
        if self.size[parent] < self.size[son] {
            std::mem::swap(&mut parent, &mut son);
        }
        self.f[son] = parent;
        self.size[parent] += self.size[son];
        self.size[son] = 0;
        self.has_ring[parent] |= self.has_ring[son];
        true
    }
}

/// 两个数字之间有一条无向边，用图/树的角度思考
/// 结论：如果一个连通块有环，则可选的数是n，否则n-1
fn union_find<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
    const MAX: usize = 400001;
    let mut us = UnionFind::new(MAX);
    let mut seen = vec![false; MAX];
    let len: usize = scanner.token();
    for _ in 0..len {
        let a: usize = scanner.token();
        let b: usize = scanner.token();
        seen[a] = true;
        seen[b] = true;
        us.union(a, b);
    }
    let mut result = 0;
    for i in 0..MAX {
        if seen[i] && us.size[i] > 0 {
            result += us.size[i];
            if !us.has_ring[i] {
                result -= 1;
            }
        }
    }

    writeln!(out, "{}", result).unwrap();
}

fn main() {
    union_find(Scanner::new(BufReader::new(stdin())), &mut stdout())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "4\n1 2\n1 3\n4 2\n2 3\n",
            "4\n",
            "2\n111 111\n111 111\n",
            "1\n",
            "12\n5 2\n5 6\n1 2\n9 7\n2 7\n5 5\n4 2\n6 7\n2 2\n7 8\n9 7\n1 8\n",
            "8\n",
        ];
        let functions = vec![
            union_find
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
