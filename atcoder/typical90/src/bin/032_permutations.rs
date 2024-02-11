use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m],
    }
    let mut s = HashSet::<(usize, usize)>::new();
    for (x, y) in xy {
        s.insert((x, y));
        s.insert((y, x));
    }

    let mut min_score = usize::MAX;
    for v in (0..n).permutations(n) {
        if v.windows(2).any(|v| s.contains(&(v[0], v[1]))) {
            continue;
        }
        let score = v.iter().enumerate().fold(0, |acc, (j, &i)| acc + a[i][j]);
        min_score = min_score.min(score);
    }
    if min_score == usize::MAX {
        println!("-1");
    } else {
        println!("{min_score}");
    }
}
