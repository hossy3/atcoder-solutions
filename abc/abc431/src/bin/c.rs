use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        h: [usize; n],
        b: [usize; m],
    }

    let mut set = BTreeSet::new();
    for (i, &h) in h.iter().enumerate() {
        set.insert((h, i));
    }

    for &b in b.iter().sorted() {
        if let Some(&(h, _)) = set.first() {
            if h <= b {
                set.pop_first();
            }
        }
    }
    let yes = n - set.len() >= k;
    println!("{}", if yes { "Yes" } else { "No" });
}
