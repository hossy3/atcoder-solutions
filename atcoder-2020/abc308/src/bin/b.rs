use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [Chars; n],
        d: [Chars; m],
        p: [usize; m + 1],
    }
    let mut map = HashMap::new();
    for i in 0..m {
        map.insert(&d[i], p[i + 1]);
    }
    let result: usize = c.iter().map(|x| map.get(x).unwrap_or(&p[0])).sum();
    println!("{}", result);
}
