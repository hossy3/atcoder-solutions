use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        k: [usize; q],
    }
    let mut map = BTreeMap::new();
    map.insert(1, 0);
    for (i, &x) in a.iter().enumerate() {
        map.insert(x - i, x);
    }
    for &x in &k {
        if let Some((&i, &x0)) = map.range(..=x).last() {
            let result = x0 + x - i + 1;
            println!("{}", result);
        }
    }
}
