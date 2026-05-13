use proconio::input;

/// ナップサック DP (Knapsack DP)
fn knapsack_dp(w: usize, wv: &[(usize, usize)]) -> usize {
    let v_sum = wv.iter().map(|(_, v)| v).sum::<usize>();
    let mut state = vec![usize::MAX; v_sum + 1]; // state[価値] = 最小重さ
    state[0] = 0;
    for &(w0, v0) in wv {
        for i in (v0..=v_sum).rev() {
            state[i] = state[i].min(state[i - v0].saturating_add(w0));
        }
    }
    let result = (0..=v_sum).rfind(|&i| state[i] <= w).unwrap();
    result
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
