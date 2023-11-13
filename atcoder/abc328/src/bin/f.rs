use std::collections::{HashMap, HashSet};

use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        abd: [(Usize1, Usize1, i64); q],
    }

    let mut uf = Dsu::new(n);
    let mut map = HashMap::new();
    let mut v = vec![0i64; n];
    let mut results = vec![];

    for (i, &(a, b, d)) in abd.iter().enumerate() {
        if uf.same(a, b) {
            if v[a] - v[b] == d {
                results.push(i + 1);
            }
            continue;
        }

        let offset = v[a] - v[b] - d;
        let a = uf.leader(a);
        let b = uf.leader(b);
        let a0 = HashSet::from([a]);
        let b0 = HashSet::from([b]);
        let set_a = map.get(&a).unwrap_or(&a0);
        let set_b = map.get(&b).unwrap_or(&b0);
        if set_a.len() < set_b.len() {
            for &i in set_a {
                v[i] -= offset;
            }
        } else {
            for &i in set_b {
                v[i] += offset;
            }
        }

        uf.merge(a, b);
        results.push(i + 1);

        let mut a0 = map.remove(&a).unwrap_or(HashSet::from([a]));
        let mut b0 = map.remove(&b).unwrap_or(HashSet::from([b]));
        if a0.len() < b0.len() {
            for i in a0 {
                b0.insert(i);
            }
            map.insert(uf.leader(b), b0);
        } else {
            for i in b0 {
                a0.insert(i);
            }
            map.insert(uf.leader(a), a0);
        }
    }

    let result = results.iter().join(" ");
    println!("{result}");
}
