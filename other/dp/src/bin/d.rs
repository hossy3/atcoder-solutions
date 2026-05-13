use proconio::input;

/// ナップサック DP (Knapsack DP)
fn knapsack_dp(w: usize, wv: &[(usize, usize)]) -> usize {
    let mut state = vec![0; w + 1];
    for &(w0, v0) in wv {
        for i in (w0..=w).rev() {
            state[i] = state[i].max(state[i - w0] + v0);
        }
    }
    state[w]
}

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n]
    }
    let result = knapsack_dp(w, &wv);
    println!("{result}");
}
