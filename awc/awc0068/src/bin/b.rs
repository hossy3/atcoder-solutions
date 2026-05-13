use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        lr: [(Usize1, Usize1); m],
    }

    let mut cum = vec![0; n + 1];
    for (i, &a) in a.iter().enumerate() {
        cum[i + 1] = cum[i] + a;
    }

    let mut min = usize::MAX;
    let mut max = 0;
    for (l, r) in lr {
        let x = cum[r + 1] - cum[l];
        min = min.min(x);
        max = max.max(x);
    }

    let result = max - min;
    println!("{result}");
}
