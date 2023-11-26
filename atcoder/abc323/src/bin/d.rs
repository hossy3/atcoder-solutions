use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        sc: [(usize, usize); n],
    }
    let mut map = BTreeMap::new();
    for (s, c) in sc {
        map.insert(s, c);
    }

    let mut count = 0;
    while let Some((s, c)) = map.pop_first() {
        if c > 1 {
            *map.entry(s * 2).or_insert(0) += c / 2;
        }
        if c % 2 == 1 {
            count += 1;
        }
    }
    println!("{count}");
}
