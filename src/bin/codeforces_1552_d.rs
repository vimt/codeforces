//! D. Array Differentiation
//! https://codeforces.com/problemset/status/1552/problem/D
//! https://www.luogu.com.cn/problem/solution/CF1552D
//! 如果存在一个长为 n 的数组 b，对于任意 i，都存在 j 和 k，使得 a[i]=b[j]-b[k]，则输出 YES，否则输出 NO。

use codeforces::scanner::Scanner;

mod graph {
    use std::io::{BufRead, Write};

    use super::*;

    /// 把b序列看成点权，把a序列看成边权，那么当构成一个环的时候b序列是满足条件的
    /// 假如a1=b1-b2, a2=b2-b3，那么如果a3=b3-b1，序列a的三个数，只用b的三个数就构造完了
    /// 进一步发现 a1+a2+a3=0，所以当 xa1+xa2+...+xan=0 (x是符号）时，满足条件 (b1-b2)+(b2-b3)+...+(bm-b1) = 0
    /// 给a序列分配符号，加/减/不取，和为0时即认为可以构造成功
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let a: Vec<i32> = (0..len).map(|_| scanner.next()).collect();
            fn check(a: &Vec<i32>, i: usize, sum: i32, ok: bool, result: &mut bool) {
                if i == a.len() || *result {
                    if sum == 0 { *result = ok; }
                    return;
                }
                check(a, i + 1, sum, ok, result);
                check(a, i + 1, sum - a[i], true, result);
                check(a, i + 1, sum + a[i], true, result);
            }
            let mut result = false;
            check(&a, 0, 0, false, &mut result);
            if result { puts!("YES"); } else { puts!("NO"); }
        };
        for _ in 0..t {
            go();
        }
    }

    /// 三进制枚举：78ms，可能除法耗时长？
    pub fn solve2<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let a: Vec<i32> = (0..len).map(|_| scanner.next()).collect();
            let total = 3usize.pow(len as u32);
            for i in 1..total {
                let mut t = i;
                let mut sum = 0;
                for &num in &a {
                    match t % 3 {
                        1 => sum += num,
                        2 => sum -= num,
                        _ => ()
                    }
                    t /= 3;
                }
                if sum == 0 {
                    puts!("YES");
                    return;
                }
            }
            puts!("NO");
        };
        for _ in 0..t {
            go();
        }
    }
}

mod baoli {
    use std::collections::HashSet;
    use std::io::{BufRead, Write};

    use super::*;

    /// 维护b所有可能的差值
    /// 来自群友
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let a: Vec<i32> = (0..len).map(|_| scanner.next()).collect();
            let mut set = HashSet::with_capacity(1 << len);
            set.insert(0);
            let mut j = 1; // b长度
            for num in a {
                if !set.contains(&num) {
                    if j > len { break; }
                    // b添加num
                    let mut b = Vec::with_capacity(set.len());
                    for &i in &set {
                        b.push(i - num);
                        b.push(i + num);
                    }
                    set.extend(b);
                    j += 1;
                }
            }
            if j <= len {
                puts!("YES");
            } else {
                puts!("NO");
            }
        };
        for _ in 0..t {
            go();
        }
    }
}

mod lingcha {
    use std::collections::HashSet;
    use std::io::{BufRead, Write};

    use super::*;

    /// https://www.luogu.com.cn/blog/endlesscheng/solution-cf1552d
    /// b为a[..n-1]的前缀和，那么如果存在 a[n] = b[j] - b[i] = a[i+1] + .. + a[j]，就存在b
    /// 判断 a 中是否存在两个子集，其子集和相等。
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let a: Vec<i32> = (0..len).map(|_| scanner.next()).collect();
            let mut set = HashSet::with_capacity(1 << len);
            for i in 0..1 << len {
                let mut sum = 0;
                for (j, &num) in a.iter().enumerate() {
                    if i >> j & 1 == 1 {
                        sum += num;
                    }
                }
                set.insert(sum);
            }
            if set.len() < 1 << len {
                puts!("YES");
            } else {
                puts!("NO");
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
    graph::solve(&mut Scanner::new(stdin), &mut stdout)
}

#[cfg(debug)]
fn main() {
    use codeforces::solves;
    use codeforces::tester::Tester;
    let t = Tester::new(solves!(graph, baoli, lingcha));
    t.test("5\n5\n4 -7 -1 5 10\n1\n0\n3\n1 10 100\n4\n-3 2 10 2\n9\n25 -171 250 174 152 242 100 -205 -258\n",
           "YES\nYES\nNO\nYES\nYES\n");
}
