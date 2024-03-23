use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    }
    let mut set = BTreeSet::new();
    for x in a {
        if x <= k {
            set.insert(x);
        }
    }
    let mut sum = k * (k + 1) / 2;
    for x in set {
        sum -= x;
    }
    println!("{sum}");
}
