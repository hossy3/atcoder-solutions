use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        bc: [(usize, usize); q],
    }
    let mut map = HashMap::new();
    for x in a {
        *map.entry(x).or_insert(0usize) += 1;
    }
    let mut cur: usize = map.iter().map(|(&k, &count)| k * count).sum();
    for (b, c) in bc {
        if let Some(&count) = map.get(&b) {
            map.remove(&b);
            *map.entry(c).or_insert(0usize) += count;
            cur -= b * count;
            cur += c * count;
        }
        println!("{cur}");
    }
}
