use proconio::input;

fn f(mut n: u128) -> (u128, bool) {
    if n == 1 {
        return (0, true);
    }
    n -= 1;

    for k in 0..18 {
        let k10: u128 = 10u128.pow(k);
        if k10 + n <= k10 * 10 {
            return (k10 + n - 1, true);
        }
        n -= k10 * 9;

        if k10 + n <= k10 * 10 {
            return (k10 + n - 1, false);
        }
        n -= k10 * 9;
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(f(1), (0, true));
        assert_eq!(f(2), (1, true));
        assert_eq!(f(10), (9, true));
        assert_eq!(f(11), (1, false));
        assert_eq!(f(19), (9, false));
        assert_eq!(f(20), (10, true));
        assert_eq!(f(46), (36, true));
        assert_eq!(f(1000000000000000000), (900000000000000000, true));
    }
}

fn main() {
    input! {
        n: u128,
    }
    let (x, b) = f(n);
    let mut result = x;
    let mut x0 = x;
    if b {
        x0 /= 10;
    }
    while x0 > 0 {
        result = result * 10 + x0 % 10;
        x0 /= 10;
    }
    println!("{result}");
}
