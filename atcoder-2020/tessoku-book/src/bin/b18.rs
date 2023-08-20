use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    map.insert(0, Vec::new());

    for (i, &a) in a.iter().enumerate() {
        let keys = map.keys().cloned().collect_vec();
        for &x in &keys {
            let y = a + x;
            if !map.contains_key(&y) {
                let mut v = map[&x].clone();
                v.push(i + 1);
                map.insert(y, v);
            }
        }
    }

    if let Some(v) = map.get(&s) {
        let result = v.iter().join(" ");
        println!("{}", v.len());
        println!("{}", result);
    } else {
        println!("{}", -1);
    };
}
