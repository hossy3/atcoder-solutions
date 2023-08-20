use std::collections::BTreeSet;
use std::ops::Bound::{Included, Unbounded};

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut set = BTreeSet::new();

    for _ in 0..q {
        input! {
            k: usize,
            x: usize,
        }

        match k {
            1 => {
                set.insert(x);
            }
            2 => {
                set.remove(&x);
            }
            _ => {
                if let Some(y) = set.range((Included(x), Unbounded)).next() {
                    println!("{}", y);
                } else {
                    println!("{}", -1);
                }
            }
        }
    }
}
