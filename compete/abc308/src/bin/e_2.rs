use proconio::{input, marker::Chars};

fn mex(m: usize, e: usize, x: usize) -> usize {
    (0..3).find(|&i| m != i && e != i && x != i).unwrap_or(3)
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }

    let mut m = vec![[0, 0, 0]; n]; // 0, 1, 2
    for i in 0..n {
        if i > 0 {
            m[i] = m[i - 1];
        }
        if s[i] == 'M' {
            m[i][a[i]] += 1;
        }
    }

    let mut x = vec![[0, 0, 0]; n]; // 0, 1, 2
    for i in (0..n).rev() {
        if i < n - 1 {
            x[i] = x[i + 1];
        }
        if s[i] == 'X' {
            x[i][a[i]] += 1;
        }
    }

    let mut result = 0usize;
    for i in 0..n {
        if s[i] != 'E' {
            continue;
        }
        for j in 0..3 {
            for k in 0..3 {
                result += m[i][j] * x[i][k] * mex(j, a[i], k);
            }
        }
    }
    println!("{}", result);
}
