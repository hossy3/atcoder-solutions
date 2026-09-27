use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut sum = vec![0usize; n + 1]; // a の累積和
    let mut map = HashMap::new(); // 前回同じ値が現れたときの最大スコア
    map.insert(0usize, 0usize);
    let mut state = vec![0usize; n + 1]; // DP A[..i] を分割したときの最大スコア
    for (i, &x) in a.iter().enumerate() {
        sum[i + 1] = (sum[i] + x) % k;
        if let Some(&j) = map.get(&sum[i + 1]) {
            state[i + 1] = (j + 1).max(state[i]);
        } else {
            state[i + 1] = state[i];
        }
        map.insert(sum[i + 1], state[i + 1]);
    }

    let result = state[n];
    println!("{result}");
}
