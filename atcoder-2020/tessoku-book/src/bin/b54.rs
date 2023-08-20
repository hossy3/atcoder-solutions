use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        a: [usize],
    }
    let mut map = HashMap::new();
    for a in a {
        if let Some(&i) = map.get(&a) {
            map.insert(a, i + 1);
        } else {
            map.insert(a, 1usize);
        }
    }
    let result: usize = map.values().map(|x| x * (x - 1) / 2).sum();
    println!("{}", result);
}
