use proconio::input;

fn f(a: i64, m: i64, x: i64) -> i64 {
    if x < a && (x - a) % m != 0 {
        (x - a) / m - 1
    } else {
        (x - a) / m
    }
}

#[test]
fn test_func() {
    assert_eq!(f(5, 3, 10), 1);
    assert_eq!(f(5, 3, 9), 1);
    assert_eq!(f(5, 3, 8), 1);
    assert_eq!(f(5, 3, 7), 0);
    assert_eq!(f(5, 3, 6), 0);
    assert_eq!(f(5, 3, 5), 0);
    assert_eq!(f(5, 3, 4), -1);
    assert_eq!(f(5, 3, 3), -1);
    assert_eq!(f(5, 3, 2), -1);
    assert_eq!(f(5, 3, 1), -2);
    assert_eq!(f(5, 3, 0), -2);
    assert_eq!(f(5, 3, -1), -2);
}

fn main() {
    input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64,
    }
    let result = f(a, m, r) - f(a, m, l - 1);
    println!("{result}");
}
