use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [usize; m],
    }

    let mut result = 0usize;
    for _ in 0..n {
        input! {
            k: usize,
            p: [Usize1; k],
        }

        let mut map = HashMap::new();
        for &x in &p {
            *map.entry(x).or_insert(0) += 1;
        }

        for (k, v) in map {
            if c[k] >= v {
                result += v;
            }
        }
    }

    println!("{result}");
}
