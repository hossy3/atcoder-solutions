use proconio::input;

/// 区間DP (Interval DP)
fn interval_dp(a: &[usize]) -> usize {
    let n = a.len();

    let mut cum = vec![0; n + 1];
    for i in 1..=n {
        cum[i] = cum[i - 1] + a[i - 1];
    }

    let mut state = vec![vec![usize::MAX; n + 1]; n + 1];
    for i in 0..n {
        state[i][i + 1] = 0;
    }
    for len in 2..=n {
        for i0 in 0..=(n - len) {
            let i1 = i0 + len;
            let mut min = usize::MAX;
            for j in 1..len {
                let i2 = i0 + j;
                min = min.min(state[i0][i2] + state[i2][i1]);
            }
            state[i0][i1] = min + cum[i1] - cum[i0];
        }
    }
    state[0][n]
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let result = interval_dp(&a);
    println!("{result}");
}
