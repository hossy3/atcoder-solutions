use ac_library::{Max, Min, Segtree};
use proconio::input;

/// 区間DP (Interval DP)
fn interval_dp(k: usize, a: &[usize]) -> usize {
    let n = a.len();

    let segtree_min: Segtree<Min<_>> = a.iter().copied().collect();
    let segtree_max: Segtree<Max<_>> = a.iter().copied().collect();

    let mut state = vec![vec![usize::MAX; k + 1]; n + 1];
    state[0][0] = 0;
    for i in 0..n {
        for j in 0..k {
            for i0 in 0..=i {
                if state[i0][j] == usize::MAX {
                    continue;
                }
                state[i + 1][j + 1] = state[i + 1][j + 1]
                    .min(state[i0][j] + segtree_max.prod(i0..=i) - segtree_min.prod(i0..=i));
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
