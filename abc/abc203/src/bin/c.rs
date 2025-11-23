use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        ab: [(usize, usize); n],
    }
    let mut map = BTreeMap::new();
    for &(a, b) in &ab {
        *map.entry(a).or_insert(0) += b;
    }
    let mut i = 0usize;
    for (&a, &b) in &map {
        if i + k < a {
            break;
        }
        k -= a - i;
        k += b;
        i = a;
    }
    println!("{}", i + k);
}
