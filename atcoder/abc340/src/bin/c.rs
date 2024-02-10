use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut map = BTreeMap::new();
    map.insert(n, 1usize);

    let mut result = 0;
    while let Some((k, x)) = map.pop_last() {
        if k == 1 {
            break;
        }
        result += k * x;
        let k0 = k / 2;
        let k1 = k - k0;
        *map.entry(k0).or_insert(0) += x;
        *map.entry(k1).or_insert(0) += x;
    }

    println!("{result}");
}
