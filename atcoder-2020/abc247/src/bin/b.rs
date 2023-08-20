use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        let s = st[i].0.as_str();
        let t = st[i].1.as_str();
        *map.entry(s).or_insert(0) += 1;
        if s != t {
            *map.entry(t).or_insert(0) += 1;
        }
    }
    let yes = (0..n).all(|i| {
        *map.get(st[i].0.as_str()).unwrap() == 1 || *map.get(st[i].1.as_str()).unwrap() == 1
    });
    println!("{}", if yes { "Yes" } else { "No" });
}
