use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut v = vec![0usize; n];
    for i in a {
        v[i] += 1;
    }

    // 出現個数ごとにまとめる
    let mut map = HashMap::new();
    for x in v {
        if x > 1 {
            *map.entry(x).or_insert(0usize) += 1;
        }
    }

    let mut result = 0usize;
    for (count0, count1) in map {
        result += ((count0 * (count0 - 1) / 2) * (n - count0)) * count1;
    }
    println!("{result}");
}
