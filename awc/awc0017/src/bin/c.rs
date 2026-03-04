use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
        lr: [(Usize1, Usize1); q],
    }

    let mut cum = vec![0usize; n];
    for i in 0..(n - 1) {
        cum[i + 1] = cum[i] + if c[i] == c[i + 1] { 1 } else { 0 };
    }

    for (l, r) in lr {
        let result = cum[r] - cum[l];
        println!("{result}");
    }
}
