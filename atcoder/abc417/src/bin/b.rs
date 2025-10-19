use std::collections::BTreeSet;

use itertools::{join, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut set = BTreeSet::new();
    for (i, &a) in a.iter().enumerate() {
        set.insert((a, i));
    }
    for &b in &b {
        if let Some(&pair) = set.range((b, 0)..(b + 1, 0)).next() {
            set.remove(&pair);
        }
    }
    let result = set.iter().map(|&(a, _)| a).join(" ");
    println!("{result}");
}
