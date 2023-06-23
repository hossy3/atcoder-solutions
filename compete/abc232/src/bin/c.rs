use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); m],
    }
    let mut set = HashSet::new();
    for &ab in &ab {
        set.insert(ab);
    }
    let yes = (0..n).permutations(n).any(|v| {
        cd.iter().all(|&(c, d)| {
            let x = v[c].min(v[d]);
            let y = v[c].max(v[d]);
            set.contains(&(x, y))
        })
    });
    println!("{}", if yes { "Yes" } else { "No" });
}
