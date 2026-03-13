use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut set = BTreeSet::new();
    for (i, &a) in a.iter().enumerate() {
        set.insert((a, i));
    }

    for _ in 0..q {
        input! {
            k: usize,
            b: [Usize1; k],
        }

        for &b in &b {
            set.remove(&(a[b], b));
        }

        let (result, _) = *set.iter().next().unwrap();
        println!("{result}");

        for &b in &b {
            set.insert((a[b], b));
        }
    }
}
