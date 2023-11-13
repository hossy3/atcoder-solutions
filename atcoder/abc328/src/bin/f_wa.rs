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
    let mut v = vec![i64::MIN; n];
    let mut results = vec![];

    for (i, &(a, b, d)) in abd.iter().enumerate() {
        if uf.same(a, b) {
            if v[a] - v[b] == d {
                results.push(i + 1);
            }
            continue;
        }

        if v[a] == i64::MIN && v[b] == i64::MIN {
            v[a] = d;
            v[b] = 0;
        } else if v[a] == i64::MIN {
            v[a] = v[b] + d;
        } else if v[b] == i64::MIN {
            v[b] = v[a] - d;
        } else {
            let offset = v[a] - v[b] - d;
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
        }

        uf.merge(a, b);
        results.push(i + 1);

        let a0 = map.remove(&a).unwrap_or(HashSet::from([a]));
        let b0 = map.remove(&b).unwrap_or(HashSet::from([b]));
        let a1: HashSet<usize> = a0.union(&b0).map(|&x| x).collect();
        map.insert(uf.leader(a), a1);
    }

    let result = results.iter().join(" ");
    println!("{result}");
}
