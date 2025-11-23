use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _: usize,
        _: usize,
        n: usize,
        pq: [(usize, usize); n],
        a: [usize],
        b: [usize],
    }
    let mut map = HashMap::new();
    for &(p, q) in &pq {
        let x = a.binary_search(&p).unwrap_err();
        let y = b.binary_search(&q).unwrap_err();
        *map.entry((x, y)).or_insert(0) += 1;
    }
    let filled = map.len() == (a.len() + 1) * (b.len() + 1);
    let min = if filled {
        *map.iter().map(|x| x.1).min().unwrap()
    } else {
        0
    };
    let max = map.iter().map(|x| x.1).max().unwrap();
    println!("{} {}", min, max);
}
