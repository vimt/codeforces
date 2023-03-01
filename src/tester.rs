use std::fmt::{Debug, Display, Formatter};
use crate::scanner::Scanner;

type JudgeFunc = fn(tc: Testcase);
type SolveFunc = fn(&mut Scanner<&'static [u8]>, &mut Vec<u8>);

pub struct Tester {
    judge: JudgeFunc,
    functions: Vec<(&'static str, SolveFunc)>,
}

impl Tester {
    pub fn new(functions: Vec<(&'static str, SolveFunc)>) -> Self {
        fn judge(tc: Testcase) {
            tc.assert_eq(tc.expect, tc.output)
        }
        return Self::with_judge(functions, judge);
    }

    pub fn with_judge(functions: Vec<(&'static str, SolveFunc)>, judge: JudgeFunc) -> Self {
        return Self {
            judge,
            functions,
        };
    }

    pub fn test(&self, input: &'static str, expect: &str) {
        for (name, func) in &self.functions {
            let mut output = Vec::new();
            func(&mut Scanner::new(input.as_bytes()), &mut output);
            let tc = Testcase { func: name, input, output: std::str::from_utf8(&output).unwrap(), expect, ..Default::default() };
            (self.judge)(tc);
        }
    }
}

#[derive(Default)]
pub struct Testcase<'a> {
    pub func: &'a str,
    pub input: &'a str,
    pub expect: &'a str,
    pub output: &'a str,
    input_sc: Option<Scanner<&'a [u8]>>,
    output_sc: Option<Scanner<&'a [u8]>>,
    expect_sc: Option<Scanner<&'a [u8]>>,
}

impl<'a> Testcase<'a> {
    pub fn assert_eq<T: Eq + Debug>(&self, a: T, b: T) {
        assert_eq!(a, b, "{}", self)
    }

    pub fn input_scanner(&mut self) -> &mut Scanner<&'a [u8]> {
        if let None = self.input_sc {
            self.input_sc = Some(Scanner::new(self.input.as_bytes()));
        }
        self.input_sc.as_mut().unwrap()
    }
    pub fn output_scanner(&mut self) -> &mut Scanner<&'a [u8]> {
        if let None = self.output_sc {
            self.output_sc = Some(Scanner::new(self.output.as_bytes()));
        }
        self.output_sc.as_mut().unwrap()
    }
    pub fn expect_scanner(&mut self) -> &mut Scanner<&'a [u8]> {
        if let None = self.expect_sc {
            self.expect_sc = Some(Scanner::new(self.expect.as_bytes()));
        }
        self.expect_sc.as_mut().unwrap()
    }
}

impl Display for Testcase<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "mod:{}\ninput:\n{}\noutput:\n{}\nexpect:\n{}", self.func, self.input, self.output, self.expect)
    }
}

#[macro_export]
macro_rules! solves {
    ($($r:tt),*) => {
        vec![$((stringify!($r), $r::solve)),*]
    };
}