use proconio::input;

fn combination(n: usize, r: usize) -> usize {
    let r = r.min(n - r);
    (1..=r).fold(1, |acc, x| acc * (n - x + 1) / x)
}

#[test]
fn test_func() {
    assert_eq!(combination(1, 0), 1);
    assert_eq!(combination(1, 1), 1);

    assert_eq!(combination(2, 0), 1);
    assert_eq!(combination(2, 1), 2);
    assert_eq!(combination(2, 2), 1);

    assert_eq!(combination(3, 0), 1);
    assert_eq!(combination(3, 1), 3);
    assert_eq!(combination(3, 2), 3);
    assert_eq!(combination(3, 3), 1);

    assert_eq!(combination(4, 0), 1);
    assert_eq!(combination(4, 1), 4);
    assert_eq!(combination(4, 2), 6);
    assert_eq!(combination(4, 3), 4);
    assert_eq!(combination(4, 4), 1);
}

fn main() {
    input! {
        n: usize,
        r: usize,
    }
    let result = combination(n, r);
    println!("{}", result);
}
