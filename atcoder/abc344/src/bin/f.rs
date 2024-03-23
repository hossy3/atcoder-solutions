use std::collections::BTreeMap;

use proconio::input;

fn div_ceil(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

fn main() {
    input! {
        n: usize,
        p: [[usize; n]; n],
        r: [[usize; n - 1]; n],
        d: [[usize; n]; n - 1],
    }

    let mut dp = vec![BTreeMap::new(); n];

    // row 0
    dp[0].insert(p[0][0], (0usize, 0usize)); // (休憩して得られる最大得点, (歩数, 残金))
    for i in 0..(n - 1) {
        let mut x = dp[i + 1].clone();
        for (&rest, &(step, money)) in &dp[i] {
            let k = div_ceil(r[0][i].saturating_sub(money), rest);
            let step = step + k + 1;
            let money = money + k * rest - r[0][i];
            let rest = rest.max(p[0][i + 1]);
            if let Some(&(step0, money0)) = x.get(&rest) {
                if step0 > step || (step0 == step && money0 < money) {
                    x.insert(rest, (step, money));
                }
            } else {
                x.insert(rest, (step, money));
            }
        }
        dp[i + 1] = x;
    }

    // row 1, ...
    for i in 0..(n - 1) {
        let prev = dp;
        dp = vec![BTreeMap::new(); n];

        for j in 0..n {
            let mut x = dp[j].clone();
            for (&rest, &(step, money)) in &prev[j] {
                let k = div_ceil(d[i][j].saturating_sub(money), rest);
                let step = step + k + 1;
                let money = money + k * rest - d[i][j];
                let rest = rest.max(p[i + 1][j]);
                if let Some((step0, money0)) = x.get_mut(&rest) {
                    if *step0 > step || (*step0 == step && *money0 < money) {
                        *step0 = step;
                        *money0 = money;
                    }
                } else {
                    x.insert(rest, (step, money));
                }
            }
            dp[j] = x;
        }

        for j in 0..(n - 1) {
            let mut x = dp[j + 1].clone();
            for (&rest, &(step, money)) in &dp[j] {
                let k = div_ceil(r[i + 1][j].saturating_sub(money), rest);
                let step = step + k + 1;
                let money = money + k * rest - r[i + 1][j];
                let rest = rest.max(p[i + 1][j + 1]);
                if let Some((step0, money0)) = x.get_mut(&rest) {
                    if *step0 > step || (*step0 == step && *money0 < money) {
                        *step0 = step;
                        *money0 = money;
                    }
                } else {
                    x.insert(rest, (step, money));
                }
            }
            dp[j + 1] = x;
        }
    }

    let result = dp[n - 1].iter().map(|(_, &(step, _))| step).min().unwrap();
    println!("{result}");
}
