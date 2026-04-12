use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        q: usize,
        th: [(u8, usize); q],
    }

    let mut set = BTreeSet::new();
    for (i, &(t, h)) in th.iter().enumerate() {
        if t == 1 {
            set.insert((h, i));
        } else {
            while let Some(&(h0, i0)) = set.first() {
                if h0 > h {
                    break;
                }
                set.remove(&(h0, i0));
            }
        }
        println!("{}", set.len());
    }
}
