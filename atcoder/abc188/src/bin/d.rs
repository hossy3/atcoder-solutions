use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: i64,
        abc: [(i64, i64, i64); n],
    }
    let mut map = BTreeMap::new();
    for &(a, b, c) in &abc {
        *map.entry(a).or_insert(0) += c;
        *map.entry(b + 1).or_insert(0) -= c;
    }

    let mut result = 0;
    let mut price = 0;
    let mut i = 0;
    for (&j, &diff) in &map {
        result += (j - i) * price.min(c);
        i = j;
        price += diff;
    }
    println!("{result}");
}
