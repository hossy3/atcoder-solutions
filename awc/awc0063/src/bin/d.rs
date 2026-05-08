use proconio::input;

/// 区間DP (Interval DP)
fn interval_dp(a: &[isize]) -> isize {
    let n = a.len();
    let mut state = vec![vec![0isize; n + 1]; n + 1];
    for i in 1..=n {
        let b = (n - i) % 2 == 0; // 太郎君のターンか
        for i0 in 0..=(n - i) {
            let i1 = i0 + i;
            if b {
                state[i0][i1] = (state[i0 + 1][i1] + a[i0]).max(state[i0][i1 - 1] + a[i1 - 1]);
            } else {
                state[i0][i1] = (state[i0 + 1][i1] - a[i0]).min(state[i0][i1 - 1] - a[i1 - 1]);
            }
        }
    }
    state[0][n]
}

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }
    let result = interval_dp(&a);
    println!("{result}");
}
