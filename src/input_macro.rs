
#[macro_export]
macro_rules! input {
    ($scanner:expr, $($r:tt)*) => {
        $crate::input_inner!{$scanner, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($scanner:expr) => {};
    ($scanner:expr, ) => {};

    ($scanner:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = $crate::read_value!($scanner, $t);
        $crate::input_inner!{$scanner $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($scanner:expr, ( $($t:tt),* )) => {
        ( $($crate::read_value!($scanner, $t)),* )
    };
    ($scanner:expr, [ $t:tt ]) => {{
        let len = $crate::read_value!($scanner, usize);
        $crate::read_value!($scanner, [$t; len])
    }};
    ($scanner:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| $crate::read_value!($scanner, $t)).collect::<Vec<_>>()
    };
    ($scanner:expr, chars) => {
        $crate::read_value!($scanner, String).chars().collect::<Vec<char>>()
    };
    ($scanner:expr, $t:ty) => {
        $scanner.next::<$t>()
    };
}
