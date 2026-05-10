use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0) += a;
    }

    let mut v = map.iter().map(|(_, &x)| x).collect::<Vec<_>>();
    v.sort_unstable();
    if v.len() <= k {
        v.clear()
    } else {
        v.truncate(v.len() - k);
    }
    let result = v.iter().sum::<usize>();
    println!("{result}");
}
