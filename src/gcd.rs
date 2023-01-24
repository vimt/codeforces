//! bench：从benchmark来看，gcd2最快。但从实际leetcode运行事件来看，gcd1 = gcd3 < gcd2

pub fn gcd1<T: Ord + Copy + Default + std::ops::Rem<Output=T>>(mut a: T, mut b: T) -> T {
    if a < b { std::mem::swap(&mut a, &mut b); }
    while b != T::default() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub fn gcd2(mut u: i32, mut v: i32) -> i32 {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();
    loop {
        v >>= v.trailing_zeros();
        if u > v {
            v ^= u;
            u ^= v;
            v ^= u;
        }
        v -= u;
        if v == 0 {
            break;
        }
    }
    u << shift
}

pub fn gcd3(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd3(b, a % b) }
}


pub fn lcm(a: i32, b: i32) -> i32 {
    a * (b / gcd2(a, b))
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;


    #[test]
    fn test_gcd() {
        assert_eq!(gcd1(6usize, 4), 2);
        assert_eq!(gcd1(6i32, 4), 2);
        assert_eq!(gcd2(6, 4), 2);
        assert_eq!(gcd2(6, 0), 6);
        assert_eq!(gcd1(6, 0), 6);
        assert_eq!(lcm(6, 4), 12);
        assert_eq!(gcd1(5702887, 3524578), gcd2(5702887, 3524578));
    }

    #[bench]
    fn bench_gcd1(b: &mut Bencher) {
        b.iter(|| gcd1(5702887, 3524578))  //46ns
    }

    #[bench]
    fn bench_gcd2(b: &mut Bencher) {
        b.iter(|| gcd2(5702887, 3524578))  //17ns
    }

    #[bench]
    fn bench_gcd3(b: &mut Bencher) {
        b.iter(|| gcd3(5702887, 3524578))  //48ns
    }
}