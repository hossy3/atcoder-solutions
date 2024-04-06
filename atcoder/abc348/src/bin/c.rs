use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(usize, usize); n],
    }
    let mut map = HashMap::new();
    for &(a, c) in &ac {
        if let Some(x) = map.get_mut(&c) {
            if *x > a {
                *x = a;
            }
        } else {
            map.insert(c, a);
        }
    }
    let result = map.values().max().unwrap();
    println!("{result}");
}
