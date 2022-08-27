use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        txa: [(usize, usize, usize); n],
    }
    let t_max = txa.iter().map(|x| x.0).max().unwrap_or(0);
    let mut h = HashMap::new();
    for &(t, x, a) in &txa {
        h.insert((t, x), a);
    }

    let mut dp = vec![0; 7]; // [0] and [6] are guards
    for t in 1..=t_max {
        let mut dp_next = dp.clone();
        for x in 0..=(t.min(4)) {
            let sc = h.get(&(t, x)).unwrap_or(&0);
            dp_next[x + 1] = dp[x].max(dp[x + 1].max(dp[x + 2])) + sc;
        }
        dp = dp_next;
    }
    let score = dp.iter().max().unwrap_or(&0);
    println!("{}", score);
}
