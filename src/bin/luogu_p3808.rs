//! 【模板】AC 自动机（简单版）
//! https://www.luogu.com.cn/record/list?status=&pid=P3808&page=1&orderBy=1

use codeforces::scanner::Scanner;

use std::collections::VecDeque;

struct AC {
    // trie树
    tr: Vec<[usize; 26]>,
    // trie树个数
    tot: usize,
    // 尾为节点u的串的个数
    e: Vec<i32>,
    // 同一字符可以匹配的其他位置信息
    fail: Vec<usize>,
}

impl AC {
    fn new(n: usize) -> Self {
        Self {
            tr: vec![[0; 26]; n],
            tot: 0,
            e: vec![0; n],
            fail: vec![0; n],
        }
    }

    pub fn insert(&mut self, s: &[u8]) {
        let mut u = 0;
        for &ch in s {
            let idx = (ch - b'a') as usize;
            if self.tr[u][idx] == 0 {
                self.tot += 1;
                self.tr[u][idx] = self.tot;
            }
            u = self.tr[u][idx];
        }
        self.e[u] += 1;
    }

    pub fn build(&mut self) {
        let mut q = VecDeque::new();
        for i in 0..26 {
            if self.tr[0][i] > 0 {
                q.push_back(self.tr[0][i]);
            }
        }
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for i in 0..26 {
                if self.tr[u][i] > 0 {
                    self.fail[self.tr[u][i]] = self.tr[self.fail[u]][i];
                    q.push_back(self.tr[u][i]);
                } else {
                    self.tr[u][i] = self.tr[self.fail[u]][i];
                }
            }
        }
    }

    pub fn query(&mut self, t: &[u8]) -> i32 {
        let mut u = 0;
        let mut result = 0;
        for &ch in t {
            let idx = (ch - b'a') as usize;
            u = self.tr[u][idx];
            let mut j = u;
            while j > 0 && self.e[j] != -1 {
                result += self.e[j];
                self.e[j] = -1;
                j = self.fail[j];
            }
        }
        result
    }
}

mod my {
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let n: usize = scanner.next();
        let mut ac = AC::new(1e6 as usize + 1);
        for _ in 0..n {
            ac.insert(scanner.next::<String>().as_bytes());
        }
        ac.build();
        let result = ac.query(scanner.next::<String>().as_bytes());
        puts!("{}", result);
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
    t.test("3\na\naa\naa\naaa",
           "3\n");
    t.test("4\na\nab\nac\nabc\nabcd",
           "3\n");
    t.test("2\na\naa\naa",
           "2\n");
}
