use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lrv: [(Usize1, Usize1, i64); q]
    }

    let mut diffs: Vec<_> = (0..(n - 1)).map(|i| a[i + 1] - a[i]).collect();
    let mut score: i64 = diffs.iter().map(|x| x.abs()).sum();

    for (l, r, v) in lrv {
        if l > 0 {
            score -= diffs[l - 1].abs();
            diffs[l - 1] += v;
            score += diffs[l - 1].abs();
        }
        if r < n - 1 {
            score -= diffs[r].abs();
            diffs[r] -= v;
            score += diffs[r].abs();
        }
        println!("{score}");
    }
}
