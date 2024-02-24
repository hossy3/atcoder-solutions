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

    let Some(min_score) = (0..n)
        .permutations(n)
        .map(|v| {
            if v.windows(2).any(|v| s.contains(&(v[0], v[1]))) {
                usize::MAX
            } else {
                v.iter().enumerate().fold(0, |acc, (j, &i)| acc + a[i][j])
            }
        })
        .min() else { unreachable!() };
    if min_score == usize::MAX {
        println!("-1");
    } else {
        println!("{min_score}");
    }
}
