use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut result = 0usize;
    let mut set = HashSet::new();
    for (u, v) in uv {
        let uv = if u > v { (v, u) } else { (u, v) };
        if u == v || !set.insert(uv) {
            result += 1;
        }
    }
    println!("{result}");
}
