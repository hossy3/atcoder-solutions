use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // 二乗が n 以下の数を列挙する
    let mut a = vec![];
    let n2 = n.isqrt();
    for i in 1..=n2 {
        a.push(i * i);
    }

    let mut map = BTreeMap::new();
    for (i, &x) in a.iter().enumerate() {
        for &y in &a[(i + 1)..] {
            *map.entry(x + y).or_insert(0usize) += 1;
        }
    }

    let results: Vec<_> = map
        .iter()
        .filter(|&(&x, &y)| x <= n && y == 1)
        .map(|(&x, _)| x)
        .collect();
    println!("{}", results.len());
    println!("{}", results.iter().join(" "));
}
