use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut result = 0;
    for v in (1..n).permutations(n - 1) {
        let mut set = HashSet::new();
        for v in v.windows(2) {
            set.insert((v[0], v[1]));
            set.insert((v[1], v[0]));
        }
        set.insert((0, v[0]));
        set.insert((v[0], 0));
        set.insert((0, v[n - 2]));
        set.insert((v[n - 2], 0));

        let mut count = 0usize;
        for &(a, b) in &ab {
            if set.contains(&(a, b)) {
                set.remove(&(a, b));
                set.remove(&(b, a));
                count += 1; // いくつ使ったかを返す
            }
        }
        result = result.max(count);
    }

    println!("{result}");
}
