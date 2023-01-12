use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        lr: [(usize, usize)],
    }

    let mut s = BTreeSet::new();
    for &(l, r) in &lr {
        s.insert((r, l));
    }

    let mut count = 0;
    let mut t = 0;
    for &(r, l) in &s {
        if l >= t {
            count += 1;
            t = r;
        }
    }
    println!("{}", count);
}
