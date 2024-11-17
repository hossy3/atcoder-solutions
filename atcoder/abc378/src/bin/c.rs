use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    let mut results: Vec<i64> = vec![];
    for (i, &x) in a.iter().enumerate() {
        if let Some(&x) = map.get(&x) {
            results.push(x);
        } else {
            results.push(-1);
        }
        map.insert(x, (i + 1) as i64);
    }
    println!("{}", results.iter().join(" "));
}
