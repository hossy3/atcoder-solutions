use proconio::{input, marker::Usize1};

fn f(n: usize, a: &[usize]) -> i64 {
    let mut m = vec![false; n * n];
    for (i, &x) in a.iter().enumerate() {
        m[x] = true;
        let r = x / n;
        let c = x % n;
        if (0..n).all(|r| m[r * n + c]) {
            return (i + 1) as i64;
        }
        if (0..n).all(|c| m[r * n + c]) {
            return (i + 1) as i64;
        }
        if r == c {
            if (0..n).all(|r| m[r * n + r]) {
                return (i + 1) as i64;
            }
        }
        if r == n - c - 1 {
            if (0..n).all(|r| m[r * n + (n - r - 1)]) {
                return (i + 1) as i64;
            }
        }
    }
    -1
}

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [Usize1; t],
    }
    let result = f(n, &a);
    println!("{result}");
}
