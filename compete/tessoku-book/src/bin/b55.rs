use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};

use proconio::input;

fn main() {
    input! {
        tx: [(usize, i64)],
    }
    let mut set = BTreeSet::new();
    for (t, x) in tx {
        if t == 1 {
            set.insert(x);
        } else {
            const MAX: i64 = 1 << 62;
            let mut result = MAX;
            if let Some(&y) = set.range((Unbounded, Included(&x))).last() {
                result = x - y;
            }
            if let Some(&y) = set.range((Excluded(&x), Unbounded)).next() {
                result = result.min(y - x);
            }
            if result == MAX {
                result = -1;
            }
            println!("{}", result);
        }
    }
}
