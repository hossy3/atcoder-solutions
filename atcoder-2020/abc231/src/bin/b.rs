use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut map = HashMap::new();
    for s in s {
        *map.entry(s).or_insert(0) += 1;
    }
    let result = map.iter().max_by_key(|x| x.1).unwrap().0;
    println!("{}", result);
}
