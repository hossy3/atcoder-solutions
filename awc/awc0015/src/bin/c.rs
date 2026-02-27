use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        pq: [(usize, usize); n],
    }

    let mut map = HashMap::new();
    for &(p, q) in &pq {
        *(map.entry(p).or_insert(HashMap::new()))
            .entry(q)
            .or_insert(0) += 1;
    }

    let mut result = 0usize;
    for map in map.values() {
        let sum: usize = map.values().sum();
        for x in map.values() {
            result += x * (sum - x);
        }
    }
    result /= 2;

    println!("{result}");
}
