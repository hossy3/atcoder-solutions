use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut count = n;
    let mut g = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {
            k: usize,
        }
        if k == 1 {
            input! {
                u: Usize1,
                v: Usize1,
            }
            if g[u].len() == 0 {
                count -= 1;
            }
            if g[v].len() == 0 {
                count -= 1;
            }
            g[u].insert(v);
            g[v].insert(u);
        } else {
            input! {
                v: Usize1,
            }
            let set = g[v].clone();
            for &u in &set {
                if g[u].len() == 1 {
                    count += 1;
                }
                g[u].remove(&v);
            }
            if g[v].len() > 0 {
                count += 1;
            }
            g[v].clear();
        }
        println!("{}", count);
    }
}
