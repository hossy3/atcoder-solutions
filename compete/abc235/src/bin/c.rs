use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize, Usize1); q],
    }
    let mut map = HashMap::new();
    for (i, &x) in a.iter().enumerate() {
        map.entry(x).or_insert(vec![]).push(i as i64 + 1);
    }
    for &(x, k) in &xk {
        let result = if let Some(v) = map.get(&x) {
            v.get(k).unwrap_or(&(-1))
        } else {
            &(-1)
        };
        println!("{}", result);
    }
}
