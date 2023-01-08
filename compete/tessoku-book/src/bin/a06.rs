use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(Usize1, Usize1); q],
    }

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    for &(l, r) in &lr {
        let result = cum[r + 1] - cum[l];
        println!("{}", result);
    }
}
