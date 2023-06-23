use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let mut map = HashMap::new();
    for i0 in 0..(n - 1) {
        let (x0, y0) = xy[i0];
        for i1 in (i0 + 1)..n {
            let (x1, y1) = xy[i1];
            if y0 == y1 {
                *map.entry((x0.min(x1), x0.max(x1))).or_insert(0usize) += 1;
            }
        }
    }
    let result: usize = map.values().map(|x| x * (x - 1) / 2).sum();
    println!("{}", result);
}
