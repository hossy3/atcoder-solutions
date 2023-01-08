use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(Usize1, Usize1); d],
    }

    let mut lmax = vec![0; n];
    lmax[0] = a[0];
    for i in 1..n {
        lmax[i] = lmax[i - 1].max(a[i]);
    }

    let mut rmax = vec![0; n];
    rmax[n - 1] = a[n - 1];
    for i in (0..(n - 1)).rev() {
        rmax[i] = rmax[i + 1].max(a[i]);
    }

    for &(l, r) in &lr {
        let result = lmax[l - 1].max(rmax[r + 1]);
        println!("{}", result);
    }
}
