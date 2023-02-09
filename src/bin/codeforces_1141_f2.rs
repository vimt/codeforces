//! F2. Same Sum Blocks (Hard)

use std::collections::HashMap;
use std::io::{stdin, stdout};
use std::io::prelude::*;
use codefoces::scanner::Scanner;

fn num1_hash_map<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
    macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
    let len: usize = scanner.next();
    let a: Vec<i32> = (0..len).map(|_| scanner.next()).collect();
    let mut map: HashMap<i32, (usize, i32)> = HashMap::new();
    let mut max_len = 0;
    let mut max_len_num = 0;
    for j in 0..len {
        let mut sum = 0;
        for i in (0..=j).rev() {
            sum += a[i];
            let (last_j, tot) = map.entry(sum).or_default();
            if i >= *last_j {
                *tot += 1;
                *last_j = j + 1;
                if *tot > max_len {
                    max_len = *tot;
                    max_len_num = sum;
                }
            }
        }
    }
    puts!("{}", max_len);
    let mut o = 0;
    for j in 0..len {
        let mut sum = 0;
        for i in (o..=j).rev() {
            sum += a[i];
            if sum == max_len_num {
                o = j + 1;
                puts!("{} {}", i+1, j+1);
                break;
            }
        }
    }
}

/// 大佬做法，77ms第一
fn num1<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
    macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
    const MO: i32 = 1635947;
    const N: i32 = 1501;
    let mut has = vec![0; (N * N) as usize];
    let mut hash = |x: i32| {
        let mut t = x;
        while t < 0 { t += MO; }
        t %= MO;
        while has[t as usize] != 0 && has[t as usize] != x {
            t = (t + 1) % MO;
        }
        has[t as usize] = x;
        return t;
    };
    let len: usize = scanner.next();
    let mut a = vec![0; len + 1];
    for i in 1..=len {
        a[i] = scanner.next();
    }
    let mut r = vec![0; (N * N) as usize];
    let mut tot = vec![0; (N * N) as usize];
    let mut pl = 0;
    let mut ans = 0;
    for j in 1..=len {
        let mut k = 0;
        for i in (1..=j).rev() {
            k += a[i];
            let t = hash(k) as usize;
            if i > r[t] {
                tot[t] += 1;
                r[t] = j;
                if ans < tot[t] {
                    ans = tot[t];
                    pl = t;
                }
            }
        }
    }
    puts!("{}", ans);
    let mut o = 0;
    for j in 1..=len {
        let mut k = 0;
        for i in (o + 1..=j).rev() {
            k += a[i];
            if k == has[pl] {
                o = j;
                puts!("{} {}", i, j);
                break;
            }
        }
    }
}

/// 常规做法498ms
fn my<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
    macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
    let mut map: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
    let len: usize = scanner.next();
    let v: Vec<i32> = (0..len).map(|_| scanner.next()).collect();
    for j in 0..len {
        let mut cur_sum = 0;
        for i in (0..=j).rev() {
            cur_sum += v[i];
            map.entry(cur_sum).or_default().push((i, j));
        }
    }
    let mut max_v = vec![];
    for (_, pos) in map {
        let mut last_end = pos[0].1;
        let mut v = vec![pos[0]];
        for (start, end) in pos.into_iter().skip(1) {
            if start > last_end {
                v.push((start, end));
                last_end = end;
            }
        }
        if v.len() > max_v.len() {
            max_v = v;
        }
    }
    puts!("{}", max_v.len());
    for (start, end) in max_v {
        puts!("{} {}", start+1, end+1);
    }
}

fn main() {
    num1_hash_map(Scanner::new(stdin().lock()), &mut stdout().lock())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use codefoces::input;
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "7\n4 1 2 2 1 5 3\n",
            "3\n7 7\n2 3\n4 5\n",
            "11\n-5 -4 -3 -2 -1 0 1 2 3 4 5\n",
            "2\n3 4\n1 1\n",
            "4\n1 1 1 1\n",
            "4\n4 4\n1 1\n2 2\n3 3\n",
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
            num1_hash_map,
            num1,
            my,
        ];
        for func in functions {
            for tc in testcases.chunks(2) {
                let mut output = Vec::new();
                func(Scanner::new(tc[0].as_bytes()), &mut output);
                input! {Scanner::new(tc[0].as_bytes()), a: [i32]}
                input! {Scanner::new(tc[1].as_bytes()), expect_len: usize}
                input! {Scanner::new(output.as_slice()), b: [(usize, usize)]}
                assert_eq!(b.len(), expect_len, "{}", tc[0]);
                let unique_num = b.into_iter().map(|(start, end)| a[start - 1..end].iter().sum::<i32>()).collect::<HashSet<i32>>().len();
                assert_eq!(unique_num, 1);
            }
        }
    }
}
