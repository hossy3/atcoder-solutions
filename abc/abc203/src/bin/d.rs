use proconio::input;

fn f(a: &[Vec<usize>], k: usize, x: usize) -> bool {
    let n = a.len();
    let mut acc = vec![vec![0usize; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            acc[i + 1][j + 1] = if a[i][j] >= x { 1 } else { 0 };
        }
    }
    for i in 0..n {
        for j in 0..n {
            acc[i + 1][j + 1] += acc[i][j + 1];
        }
    }
    for i in 0..n {
        for j in 0..n {
            acc[i + 1][j + 1] += acc[i + 1][j];
        }
    }

    let m = k * k / 2 + 1;
    for i in 0..=(n - k) {
        for j in 0..=(n - k) {
            let count = (acc[i + k][j + k] + acc[i][j]) - (acc[i][j + k] + acc[i + k][j]);
            if count < m {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_func() {
    let a = vec![vec![1, 7, 0], vec![5, 8, 11], vec![10, 4, 2]];
    assert_eq!(f(&a, 2, 1), true);
    assert_eq!(f(&a, 2, 2), true);
    assert_eq!(f(&a, 2, 3), true);
    assert_eq!(f(&a, 2, 4), true);
    assert_eq!(f(&a, 2, 5), false);
    assert_eq!(f(&a, 2, 6), false);
    assert_eq!(f(&a, 2, 7), false);
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n],
    }

    let mut l = 0;
    let mut r = 1 << 30;
    while l + 1 != r {
        let x = (l + r) / 2;
        if f(&a, k, x) {
            l = x;
        } else {
            r = x;
        }
    }
    println!("{}", l);
}
