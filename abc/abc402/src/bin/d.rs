use proconio::{input, marker::Usize1};

fn f(x: usize) -> usize {
    if x >= 2 {
        x * (x - 1) / 2
    } else {
        0
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut v = vec![0usize; n];
    for &(a, b) in &ab {
        let i = (a + b) % n;
        v[i] += 1;
    }

    let mut result = f(m);
    for &x in &v {
        result -= f(x);
    }
    println!("{result}");
}
