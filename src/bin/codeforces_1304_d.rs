//! D. Shortest and Longest LIS
//! https://codeforces.com/problemset/status/1304/problem/D?order=BY_CONSUMED_TIME_ASC
//! https://www.luogu.com.cn/problem/solution/CF1304D
//! 根据相邻数字的大小关系，构造出两个数组，一个数组的LIS最小，另一个LIS最大

use codeforces::scanner::Scanner;

mod my {
    use std::io::{BufRead, Write};
    use super::*;


    // 最短：拆分成若干上升段，那么把最大的数字分配给最左边的上升段，剩余的最大数字分配给第二个上升段，依此类推。
    // 最长：拆分成若干下降段，那么把最小的数字分配给最左边的下降段，剩余的最小数字分配给第二个下降段，依此类推。
    pub fn solve<R: BufRead, W: Write>(scanner: &mut Scanner<R>, out: &mut W) {
        macro_rules! puts_vec {($arr:tt) => (for num in $arr {let _ = write!(out,"{} ",num);} let _ = writeln!(out);)}
        let t: usize = scanner.next();
        let mut go = || {
            let len: usize = scanner.next();
            let order = scanner.next::<String>().into_bytes();
            let mut short: Vec<usize> = (1..=len).rev().collect();
            let mut long: Vec<usize> = (1..=len).collect();
            let mut i = 0;
            while i < len - 1 {
                let h = i;
                while i < len - 1 && order[i] == order[h] {
                    i += 1;
                }
                if order[h] == b'>' { long[h..=i].reverse(); } else { short[h..=i].reverse(); };
            }
            puts_vec!(short);
            puts_vec!(long);
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
    use codeforces::{solves, input};
    use codeforces::tester::{Tester, Testcase};
    fn lis(nums: &Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        for i in 1..nums.len() {
            dp[i] = (0..i).filter(|&j| nums[j] < nums[i]).map(|x| dp[x]).fold(0, i32::max) + 1;
        }
        *dp.last().unwrap()
    }
    fn verify(mut tc: Testcase) {
        let t: usize = tc.input_scanner().next();
        for _ in 0..t {
            let len: usize = tc.input_scanner().next();
            let order = tc.input_scanner().next::<String>().into_bytes();
            input! {tc.output_scanner(), short: [i32; len], long: [i32; len]}
            input! {tc.expect_scanner(), expect_short: [i32; len], expect_long: [i32; len]}
            for i in 0..len - 1 {
                assert_eq!(if short[i] > short[i + 1] { b'>' } else { b'<' }, order[i]);
                assert_eq!(if long[i] > long[i + 1] { b'>' } else { b'<' }, order[i]);
            }
            assert_eq!(lis(&short), lis(&expect_short));
            assert_eq!(lis(&long), lis(&expect_long));
        }
    }
    let t = Tester::with_judge(verify, solves!(my));
    t.test("3\n3 <<\n7 >><>><\n5 >>><\n",
           "1 2 3\n1 2 3\n5 4 3 7 2 1 6\n4 3 1 7 5 2 6\n4 3 2 1 5\n5 4 2 1 3\n");
}
