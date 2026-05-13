use proconio::input;

/// 期待値DP (Probablistic DP)
fn probablistic_dp(p: &[f64]) -> f64 {
    let n = p.len();
    let mut state = vec![0.0; n + 1];
    state[0] = 1.0;
    for (i, &p) in p.iter().enumerate() {
        let mut next = vec![0.0; n + 1];
        for (i, p0) in state[..=i].iter().enumerate() {
            next[i] += p0 * (1.0 - p);
            next[i + 1] += p0 * p;
        }
        state = next;
    }
    state[((p.len() + 1) / 2)..].iter().sum()
}

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }
    let result = probablistic_dp(&p);
    println!("{result}");
}
