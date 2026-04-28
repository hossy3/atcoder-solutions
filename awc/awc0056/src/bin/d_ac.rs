use proconio::input;

/// 区間DP (Interval DP)
fn interval_dp(k: usize, a: &[usize]) -> usize {
    let n = a.len();

    let mut state = vec![vec![usize::MAX; k + 1]; n + 1];
    state[0][0] = 0;
    for i in 0..n {
        for j in 0..k {
            for i0 in 0..=i {
                if state[i0][j] == usize::MAX {
                    continue;
                }
                let x = a[i0..=i].iter().max().unwrap() - a[i0..=i].iter().min().unwrap();
                state[i + 1][j + 1] = state[i + 1][j + 1].min(state[i0][j] + x);
            }
        }
    }

    state[n][k]
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let result = interval_dp(k, &a);
    println!("{result}");
}
