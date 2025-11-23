use proconio::{input, marker::Usize1};

fn f(a: &[usize]) -> usize {
    let n = a.len();

    let mut v0 = vec![0usize; a.len()];
    let mut v1 = vec![0usize; a.len()];
    for &x in &a[1..] {
        v1[x] += 1;
    }

    let mut result = 0;
    let mut pairs = 0;
    for i in 1..(n - 1) {
        v0[a[i - 1]] += 1;
        pairs += v1[a[i - 1]];

        v1[a[i]] -= 1;
        pairs -= v0[a[i]];

        result += pairs - v0[a[i]] * v1[a[i]];
    }
    result
}

#[test]
fn test_func() {
    assert_eq!(f(&[1, 2, 2, 2, 3, 2, 2, 2, 4]), 9);
    assert_eq!(f(&[1, 2, 2, 2, 3, 2, 2, 2, 4, 2]), 18);
}

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let result = f(&a);
    println!("{result}");
}
