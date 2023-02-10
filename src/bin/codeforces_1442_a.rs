use std::io::{stdin, stdout};
use std::io::prelude::*;
use codeforces::scanner::Scanner;

fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
    let t: usize = scanner.next();
    for _ in 0..t {
        let len: usize = scanner.next();
        let a: i32 = scanner.next();
        let mut la: i32 = a;
        let mut sum = 0;
        for _ in 1..len {
            let x = scanner.next();
            if x - la < 0 { sum += la - x; }
            la = x;
        }
        writeln!(out, "{}", if sum <= a { "YES" } else { "NO" }).unwrap();
    }
}

fn main() {
    solve(Scanner::new(stdin().lock()), &mut stdout().lock())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "4\n3\n1 2 1\n5\n11 7 9 6 8\n5\n1 3 1 3 1\n4\n5 2 1 10\n";
        let expect = "YES\nYES\nNO\nYES\n";
        let mut output = Vec::new();
        solve(Scanner::new(input.as_bytes()), &mut output);
        assert_eq!(String::from_utf8(output).unwrap(), expect);
    }
}
