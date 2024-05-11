use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    }

    let mut v = vec![None; n]; // Pi の出現場所
    for (i, &x) in p.iter().enumerate() {
        v[x] = Some(i);
    }

    let mut result = usize::MAX;
    let mut set = BTreeSet::new();
    for i in 0..(k - 1) {
        if let Some(x) = v[i] {
            set.insert(x);
        }
    }
    for i in (k - 1)..n {
        if let Some(x) = v[i] {
            set.insert(x);
        }
        if set.len() == k {
            let x = set.last().unwrap() - set.first().unwrap();
            result = result.min(x);
        }
        if let Some(x) = v[i + 1 - k] {
            set.remove(&x);
        }
    }
    println!("{result}");
}
