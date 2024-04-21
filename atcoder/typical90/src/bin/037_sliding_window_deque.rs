use std::collections::VecDeque;

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
        let mut queue = VecDeque::<usize>::new();
        for i in l..=w {
            while let Some(&j) = queue.back() {
                if prev[j] > prev[i - l] {
                    break;
                }
                queue.pop_back();
            }
            queue.push_back(i - l);
            if i > r && queue[0] == i - r - 1 {
                queue.pop_front();
            }
            let x = prev[queue[0]];
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
