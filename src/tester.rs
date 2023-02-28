use crate::scanner::Scanner;

type JudgeFunc = fn(input: &str, expect: &str, output: &str);
type SolveFunc = fn(&mut Scanner<&[u8]>, &mut Vec<u8>);

struct Tester {
    judge: JudgeFunc,
    functions: Vec<SolveFunc>,
}

impl Tester {
    fn new(functions: Vec<SolveFunc>) -> Self {
        fn judge(input: &str, expect: &str, output: &str) {
            assert_eq!(output, expect);
        }
        return Self::with_judge(functions, judge);
    }

    fn with_judge(functions: Vec<SolveFunc>, judge: JudgeFunc) -> Self {
        return Self {
            judge,
            functions,
        };
    }
}

