use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n],
    }
    let mut map = BTreeMap::new();
    for &c in &c[..k] {
        *map.entry(c).or_insert(0) += 1;
    }
    let mut result = map.len();
    for i in k..n {
        *map.entry(c[i]).or_insert(0) += 1;
        let x = c[i - k];
        if let Some(&y) = map.get(&x) {
            if y == 1 {
                map.remove(&x);
            } else {
                map.insert(x, y - 1);
            }
        }
        result = result.max(map.len());
    }
    println!("{}", result);
}
