use proconio::input;

fn f(n: usize, s: usize) -> usize {
    if s >= n * 2 {
        n * n
    } else if s >= n + 1 {
        let t = 2 * n - s;
        n * n - (t + 1) * t / 2
    } else {
        s * (s - 1) / 2
    }
}

#[test]
fn test() {
    assert_eq!(f(3, 2), 1);
    assert_eq!(f(3, 3), 3);
    assert_eq!(f(3, 4), 6);
    assert_eq!(f(3, 5), 8);
    assert_eq!(f(3, 6), 9);
    assert_eq!(f(3, 7), 9);
}

fn main() {
    input! {
        n: usize,
        s: usize,
    }
    let result = f(n, s);
    println!("{}", result);
}
