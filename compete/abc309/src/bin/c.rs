use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        ab: [(usize, i64); n],
    }
    let mut map = BTreeMap::new();
    for &(a, b) in &ab {
        *map.entry(1).or_insert(0) += b;
        *map.entry(a + 1).or_insert(0) -= b;
    }
    let mut sum = 0;
    for (&a, &b) in &map {
        sum += b;
        if sum <= k {
            println!("{}", a);
            return;
        }
    }
}
