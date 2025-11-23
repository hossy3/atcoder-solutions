use proconio::input;

// 10進数での回文その1を作る (abc → abccba)
fn f0(mut x: usize) -> usize {
    let mut k = x.ilog10() + 1; // 10進数での桁数
    let mut result = 10usize.pow(k) * x;
    while k > 0 {
        k -= 1;
        result += (x % 10) * 10usize.pow(k);
        x /= 10;
    }
    result
}

// 10進数での回文その2を作る (abc → abcba)
fn f1(mut x: usize) -> usize {
    let mut k = x.ilog10(); // 10進数での桁数
    let mut result = 10usize.pow(k) * x;
    x /= 10;
    while k > 0 {
        k -= 1;
        result += (x % 10) * 10usize.pow(k);
        x /= 10;
    }
    result
}

// A進数で回文かを調べる
fn f2(mut x: usize, a: usize) -> bool {
    let mut v = vec![];
    while x > 0 {
        v.push(x % a);
        x /= a;
    }
    (0..(v.len() / 2)).all(|i| v[i] == v[v.len() - i - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f0() {
        assert_eq!(f0(123), 123321);
        assert_eq!(f0(1), 11);
        assert_eq!(f0(100), 100001);
    }

    #[test]
    fn test_f1() {
        assert_eq!(f1(123), 12321);
        assert_eq!(f1(1), 1);
        assert_eq!(f1(100), 10001);
    }

    #[test]
    fn test_f2() {
        assert_eq!(f2(1, 10), true);
        assert_eq!(f2(100, 10), false);
        assert_eq!(f2(12321, 10), true);
        assert_eq!(f2(123321, 10), true);
    }
}

fn main() {
    input! {
        a: usize,
        n: usize,
    }

    let mut result = 0;
    for i in 1..1_000_000 {
        let x = f0(i);
        if x <= n && f2(x, a) {
            result += x;
        }

        let x = f1(i);
        if x > n {
            break;
        }
        if f2(x, a) {
            result += x;
        }
    }

    println!("{result}");
}
