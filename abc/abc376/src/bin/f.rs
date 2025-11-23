use std::collections::HashMap;

use proconio::{input, marker::Usize1};

// l を t に移動するときの方法
fn f(n: usize, l: usize, r: usize, t: usize) -> Vec<(usize, usize, usize)> {
    if l == t {
        vec![(l, r, 0)]
    } else if r == t && l < t {
        vec![
            (t, (t + n - 1) % n, (l + n - t) + 1),
            (t, (t + 1) % n, (t - l) + 1),
        ]
    } else if r == t && t < l {
        vec![
            (t, (t + n - 1) % n, (l - t) + 1),
            (t, (t + 1) % n, (t + n - l) + 1),
        ]
    } else if l < t && t < r {
        vec![
            (t, r, t - l),
            (t, (t + n - 1) % n, (l + n - t) + (r - t + 1)),
        ]
    } else if l < r && r < t {
        vec![(t, r, l + n - t), (t, (t + 1) % n, (t - l) + (t - r + 1))]
    } else if r < t && t < l {
        vec![(t, r, l - t), (t, (t + 1) % n, (t + n - l) + (t - r + 1))]
    } else if r < l && l < t {
        vec![
            (t, r, t - l),
            (t, (t + n - 1) % n, (l + n - t) + (r + n - t + 1)),
        ]
    } else if t < l && l < r {
        vec![
            (t, r, l - t),
            (t, (t + 1) % n, (t + n - l) + (t + n - r + 1)),
        ]
    } else if t < r && r < l {
        vec![
            (t, r, (t + n).abs_diff(l)),
            (t, (t + n - 1) % n, (l - t) + (r - t + 1)),
        ]
    } else {
        unreachable!()
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, Usize1); q],
    }
    let mut dp = vec![(0, 1, 0)]; // l, r, score
    for (x, t) in ht {
        let mut dp0 = HashMap::new();
        for &(l, r, score) in &dp {
            let cand: Vec<_> = if x == 'L' {
                f(n, l, r, t)
                    .iter()
                    .map(|&(l, r, s)| (l, r, score + s))
                    .collect()
            } else {
                f(n, r, l, t)
                    .iter()
                    .map(|&(r, l, s)| (l, r, score + s))
                    .collect()
            };

            for (l, r, result) in cand {
                if let Some(&result0) = dp0.get(&(l, r)) {
                    if result0 <= result {
                        continue;
                    }
                }
                dp0.insert((l, r), result);
            }
        }

        dp.clear();
        for ((l, r), s) in dp0 {
            dp.push((l, r, s));
        }
    }

    let result = dp.iter().map(|&(_, _, s)| s).min().unwrap();
    println!("{result}");
}
