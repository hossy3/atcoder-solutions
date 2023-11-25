use proconio::input;

fn f(n: usize) -> usize {
    let m = 10usize.pow((((n as f64).log10() + 1.0) / 2.0).floor() as u32);
    let n0 = n / m;
    let n1 = n % m;
    let result = if n0 >= m {
        m - 1
    } else if n0 <= n1 {
        n0
    } else {
        n0 - 1
    };
    result
}

#[test]
fn test_func() {
    assert_eq!(f(1), 0);
    assert_eq!(f(11), 1);
    assert_eq!(f(12), 1);
    assert_eq!(f(21), 1);
    assert_eq!(f(22), 2);
    assert_eq!(f(99), 9);
    assert_eq!(f(100), 9);
    assert_eq!(f(111), 9);
    assert_eq!(f(999), 9);
    assert_eq!(f(1000), 9);
    assert_eq!(f(1009), 9);
    assert_eq!(f(1010), 10);
    assert_eq!(f(1011), 10);
}

fn main() {
    input! {
        n: usize,
    }
    let result = f(n);
    println!("{}", result);
}
