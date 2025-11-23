use num_traits::Float;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }

    let mut m = vec![vec![0.0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..=i {
            m[i + 1][j] = m[i + 1][j].max(m[i][j]);
            m[i + 1][j + 1] = m[i][j] * 0.9 + p[i];
        }
    }

    let mut score = -1200.0;
    let mut bottom = 0.0;
    for i in 1..=n {
        bottom = bottom * 0.9 + 1.0;
        let s = m[n][i] / bottom - 1200.0 / (i as f64).sqrt();
        score = score.max(s);
    }
    println!("{score}");
}
