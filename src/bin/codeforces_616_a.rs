//! A. Comparing Two Long Integers
//! https://codeforces.com/contest/616/problem/A

use codeforces::scanner::Scanner;

mod my {
    use std::cmp::Ordering;
    use std::io::{BufRead, Write};
    use super::*;

    pub fn solve<R: BufRead, W: Write>(mut scanner: Scanner<R>, out: &mut W) {
        macro_rules! puts {($($format:tt)*) => (let _ = writeln!(out,$($format)*););}
        let a = scanner.next::<String>().into_bytes();
        let b = scanner.next::<String>().into_bytes();
        let mut i = 0;
        let mut j = 0;
        while i < a.len() && a[i] == b'0' { i += 1; }
        while j < b.len() && b[j] == b'0' { j += 1; }
        let alen = a.len() - i;
        let blen = b.len() - j;
        puts!("{}", match (alen, &a[i..]).cmp(&(blen, &b[j..])) {
            Ordering::Less => "<",
            Ordering::Equal => "=",
            Ordering::Greater => ">"
        });
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
            "9\n10",
            "<\n",
            "11\n10",
            ">\n",
            "00012345\n12345",
            "=\n",
            "0123\n9",
            ">\n",
            "0123\n111",
            ">\n",
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
