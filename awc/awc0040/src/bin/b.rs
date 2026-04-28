use proconio::{input, marker::Usize1};

fn main() {
    input! {
        m: usize,
        s: usize,
        mut b: [usize; m],
        n: usize,
        lr: [(Usize1, Usize1); n],
    }

    for i in 0..m {
        b[i] += s / m + if s % m > i { 1 } else { 0 };
    }
    let mut cum = vec![0usize; m + 1];
    for i in 0..m {
        cum[i + 1] = cum[i] + b[i];
    }

    for (l, r) in lr {
        let result = cum[r + 1] - cum[l];
        println!("{result}");
    }
}
