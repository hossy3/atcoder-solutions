use proconio::input;

fn f(v: &[usize]) -> usize {
    let mut count = 0;
    for i in 0..(v.len() - 1) {
        for j in (i + 1)..v.len() {
            if v[i] > v[j] {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        p: [[usize; n]; n],
    }

    let mut v0 = Vec::with_capacity(n);
    for i in 0..n {
        v0.push((0..n).map(|j| p[i][j]).max().unwrap() - 1);
    }

    let mut v1 = Vec::with_capacity(n);
    for j in 0..n {
        v1.push((0..n).map(|i| p[i][j]).max().unwrap() - 1);
    }

    let count = f(&v0) + f(&v1);
    println!("{}", count);
}
