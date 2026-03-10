use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        t: usize,
        b: [Usize1; m],
        lr: [(Usize1, Usize1); k],
    }

    let mut cum = vec![0; n + 1];
    for &b in &b {
        cum[b + 1] += 1;
    }
    for i in 0..n {
        cum[i + 1] += cum[i];
    }

    for (l, r) in lr {
        let sum = cum[r + 1] - cum[l];
        let yes = sum >= t;
        println!("{}", if yes { "YES" } else { "NO" });
    }
}
