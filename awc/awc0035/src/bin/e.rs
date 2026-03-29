use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut result = 0usize;
    let mut set = BTreeSet::new();
    for (i, &h) in h.iter().enumerate() {
        while let Some(&(h0, i0)) = set.first() {
            if h0 > h {
                break;
            }
            set.remove(&(h0, i0));
        }
        result += set.len();
        set.insert((h, i));
    }

    println!("{result}");
}
