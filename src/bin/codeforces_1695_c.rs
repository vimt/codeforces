//! C. Zero Path

use std::io::{BufReader, stdin, stdout};
use std::io::prelude::*;
use codeforces::scanner::Scanner;

fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
    let t: usize = scanner.token();
    for _ in 0..t {
        let m: usize = scanner.token();
        let n: usize = scanner.token();
        let mut grid = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                grid[i][j] = scanner.token();
            }
        }
        if m + n & 1 == 0 {
            writeln!(out, "NO").unwrap();
            continue;
        }

        for i in 1..m {
            grid[i][0] += grid[i - 1][0];
        }
        for i in 1..n {
            grid[0][i] += grid[0][i - 1];
        }
        // 最小路径和 & 最大路径和
        let mut min = grid.clone();
        let mut max = grid;
        for i in 1..m {
            for j in 1..n {
                min[i][j] = min[i - 1][j].min(min[i][j - 1]) + min[i][j];
                max[i][j] = max[i - 1][j].max(max[i][j - 1]) + max[i][j];
            }
        }
        if min[m - 1][n - 1] <= 0 && max[m - 1][n - 1] >= 0 {
            writeln!(out, "YES").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
    }
}

fn main() {
    solve(Scanner::new(BufReader::new(stdin())), &mut stdout())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let testcases = vec![
            "5\n1 1\n1\n1 2\n1 -1\n1 4\n1 -1 1 -1\n3 4\n1 -1 -1 -1\n-1 1 1 -1\n1 1 1 -1\n3 4\n1 -1 1 1\n-1 1 -1 1\n1 -1 1 1",
            "NO\nYES\nYES\nYES\nNO\n",
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
            solve
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
