use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        w: usize,
        n: usize,
        lrv: [(usize, usize, i64); n],
    }

    let mut dp = vec![i64::MIN; w + 1];
    dp[0] = 0;

    for (l, r, v) in lrv {
        let prev = dp.clone();
        let mut set = BTreeSet::new();
        for i in l..=w {
            set.insert((prev[i - l], i - l));
            if i > r {
                set.remove(&(prev[i - r - 1], i - r - 1));
            }
            let &(x, _) = set.last().unwrap();
            dp[i] = prev[i].max(x + v);
        }
    }

    let result = dp[w];
    if result >= 0 {
        println!("{result}");
    } else {
        println!("-1");
    }
}
