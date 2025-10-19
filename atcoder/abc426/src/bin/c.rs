use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(usize, usize); q],
    }

    let mut map = BTreeMap::new();
    for i in 1..=n {
        map.insert(i, 1);
    }

    for &(x, y) in &xy {
        let mut count = 0;
        while let Some((i, j)) = map.pop_first() {
            if i > x {
                map.insert(i, j);
                break;
            }
            count += j;
        }
        *map.entry(y).or_insert(0) += count;
        println!("{count}");
    }
}
