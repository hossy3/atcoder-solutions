use std::collections::HashSet;

use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut sets = vec![HashSet::new(); n];
    let mut dsu = Dsu::new(n);
    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    u: Usize1,
                    v: Usize1,
                }
                if dsu.same(u, v) {
                    continue;
                }
                let u0 = dsu.leader(u);
                let v0 = dsu.leader(v);
                let uv1 = dsu.merge(u, v);
                if sets[u0].len() > sets[v0].len() {
                    for &x in &sets[v0].clone() {
                        sets[u0].insert(x);
                    }
                    sets[v0].clear();
                    if uv1 == v0 {
                        sets.swap(u0, v0);
                    }
                } else {
                    for &x in &sets[u0].clone() {
                        sets[v0].insert(x);
                    }
                    sets[u0].clear();
                    if uv1 == u0 {
                        sets.swap(u0, v0);
                    }
                }
            }
            2 => {
                input! {
                    v: Usize1,
                }
                let v0 = dsu.leader(v);
                if !sets[v0].insert(v) {
                    sets[v0].remove(&v);
                }
            }
            3 => {
                input! {
                    v: Usize1,
                }
                let v0 = dsu.leader(v);
                let yes = sets[v0].len() > 0;
                println!("{}", if yes { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
