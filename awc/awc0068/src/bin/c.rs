use std::collections::HashSet;

use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uvc: [(Usize1, Usize1, usize); m],
    }

    let mut colors = vec![0; n];
    let mut dsu = Dsu::new(n);
    for (u, v, c) in uvc {
        let i = dsu.merge(u, v);
        colors[i] = c;
    }

    let mut set = HashSet::new();
    for i in 0..n {
        let x = colors[dsu.leader(i)];
        if x > 0 {
            set.insert(colors[dsu.leader(i)]);
        }
    }

    let result = set.len();
    println!("{result}");
}
