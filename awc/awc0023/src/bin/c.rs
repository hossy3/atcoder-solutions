use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        t: [usize; n],
        slr: [(usize, Usize1, Usize1); m],
    }

    let mut cum = vec![0usize; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + t[i];
    }

    for &(s, l, r) in &slr {
        let result = s + cum[r + 1] - cum[l];
        println!("{result}");
    }
}
