use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = HashMap::new();
    for x in &s {
        *map.entry(x).or_insert(0) += 1;
    }
    let a = map
        .iter()
        .map(|(&k, &v)| (v, k))
        .sorted()
        .collect::<Vec<_>>();
    let m = a[a.len() - 1].0; // 一番多いもの
    let results = a
        .iter()
        .filter(|&&(v, _)| v == m)
        .map(|&(_, k)| k)
        .collect::<Vec<_>>();
    for result in results {
        println!("{result}");
    }
}
