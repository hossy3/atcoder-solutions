use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; 3 * n],
    }
    let mut v = vec![vec![]; n];
    let mut map = BTreeMap::new();
    for (i, &x) in a.iter().enumerate() {
        v[x].push(i);
        if v[x].len() == 2 {
            map.insert(i, x + 1);
        }
    }
    let result = map.iter().map(|(_, &x)| x).join(" ");
    println!("{}", result);
}
