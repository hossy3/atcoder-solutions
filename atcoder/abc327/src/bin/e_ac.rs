use num_traits::Float;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }

    let mut ps = vec![1.0; n + 1]; // 分母
    for i in 0..n {
        ps[i + 1] = ps[i] * 0.9;
    }
    for i in 0..n {
        ps[i + 1] += ps[i];
    }

    let mut m = vec![vec![0.0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..=i {
            m[i + 1][j] = m[i + 1][j].max(m[i][j]);
            m[i + 1][j + 1] = m[i][j] * 0.9 + p[i];
        }
    }

    let mut score = -1200.0;
    for i in 1..=n {
        for j in 1..=i {
            let s = m[i][j] / ps[j - 1] - 1200.0 / (j as f64).sqrt();
            score = score.max(s);
        }
    }
    println!("{score}");
}
