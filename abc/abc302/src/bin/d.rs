use proconio::input;

fn f(d: i64, a: &[i64], b: &[i64]) -> i64 {
    let mut result = -1;
    let mut i = 0; // b's index
    for &a0 in a {
        while i < b.len() && b[i] <= a0 + d {
            let b0 = b[i];
            if (b0 - a0).abs() <= d {
                result = result.max(a0 + b0);
            }
            i += 1;
        }
        if i > 0 {
            i -= 1;
        }
    }
    result
}

#[test]
fn test_func() {
    assert_eq!(f(0, &[1, 2], &[1, 2]), 4);
    assert_eq!(f(0, &[1, 2], &[3, 4]), -1);
    assert_eq!(f(1, &[1, 2], &[3, 4]), 5);
    assert_eq!(f(0, &[1], &[2, 3, 4]), -1);
    assert_eq!(f(1, &[1], &[2, 3, 4]), 3);
    assert_eq!(f(2, &[1], &[2, 3, 4]), 4);
    assert_eq!(f(0, &[3, 4], &[1, 2]), -1);
    assert_eq!(f(1, &[3, 4], &[1, 2]), 5);
    assert_eq!(f(0, &[2, 3, 4], &[1]), -1);
    assert_eq!(f(1, &[2, 3, 4], &[1]), 3);
    assert_eq!(f(2, &[2, 3, 4], &[1]), 4);
}

fn main() {
    input! {
        n: usize,
        m: usize,
        d: i64,
        mut a: [i64; n],
        mut b: [i64; m],
    }
    a.sort();
    b.sort();
    let result = f(d, &a, &b);
    println!("{}", result);
}
