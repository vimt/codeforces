use std::collections::VecDeque;
use std::io::{BufReader, stdin, stdout};
use std::io::prelude::*;
use codefoces::scanner::Scanner;

fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
    static DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let t: i64 = scanner.token();
    for _ in 0..t {
        let m: usize = scanner.token();
        let n: usize = scanner.token();
        let mut grid = vec![vec![b'.'; n]; m];
        let mut good_num = 0;
        for i in 0..m {
            grid[i] = scanner.token::<String>().into_bytes();
            for j in 0..n {
                if grid[i][j] == b'G' { good_num += 1; }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == b'B' {
                    for (dx, dy) in DIR {
                        let (nx, ny) = (i as i32 + dx, j as i32 + dy);
                        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] != b'B' {
                            grid[nx as usize][ny as usize] = b'#';
                        }
                    }
                }
            }
        }
        let mut cnt = 0;
        if grid[m - 1][n - 1] != b'#' {
            grid[m - 1][n - 1] = b'#';
            let mut q = VecDeque::new();
            q.push_back((m - 1, n - 1));
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                for (dx, dy) in DIR {
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] != b'#' {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if grid[nx][ny] == b'G' { cnt += 1; }
                        grid[nx][ny] = b'#';
                        q.push_back((nx, ny));
                    }
                }
            }
        }
        if cnt == good_num {
            writeln!(out, "Yes").unwrap();
        } else {
            writeln!(out, "No").unwrap();
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
            "6\n1 1\n.\n1 2\nG.\n2 2\n#B\nG.\n2 3\nG.#\nB#.\n3 3\n#B.\n#..\nGG.\n2 2\n#B\nB.\n",
            "Yes\nYes\nNo\nNo\nYes\nYes\n",
        ];
        let functions: Vec<fn(Scanner<_>, &mut _)> = vec![
            solve
        ];
        for func in functions {
            for tc in testcases.chunks(2) {
                let input = tc[0].as_bytes();
                let expect_output = tc[1].as_bytes();
                let mut output = Vec::new();
                func(Scanner::new(input), &mut output);
                assert_eq!(output, expect_output);
            }
        }
    }
}
