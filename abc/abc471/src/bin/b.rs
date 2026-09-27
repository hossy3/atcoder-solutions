use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = HashMap::new();
    for s in s {
        *map.entry(s.to_lowercase()).or_insert(0) += 1;
    }

    let result = *map.values().max().unwrap();
    println!("{result}");
}
