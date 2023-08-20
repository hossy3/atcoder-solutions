use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut map = HashMap::new();
    map.insert(0, vec![0]);
    let mut sum = 0;
    for (i, &x) in a.iter().enumerate() {
        sum += x;
        map.entry(sum).or_insert(vec![]).push(i + 1);
    }
    if k == 0 {
        let result: usize = map.iter().map(|(_, v)| v.len() * (v.len() - 1) / 2).sum();
        println!("{}", result);
    } else {
        let mut result = 0;
        for (x0, v0) in map.iter() {
            let x1 = x0 - k;
            if let Some(v1) = map.get(&x1) {
                for i0 in v0 {
                    result += v1.binary_search(i0).unwrap_err();
                }
            }
        }
        println!("{}", result);
    }
}
