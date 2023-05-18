use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            t: usize,
            k: usize,
            a: [Usize1; k]
        }
        v.push((t, a));
    }
    let mut result = v[n - 1].0;
    let mut set = BTreeSet::new();
    for &i in &v[n - 1].1 {
        set.insert(i);
    }
    while let Some(&i) = set.iter().next_back() {
        set.remove(&i);
        result += v[i].0;
        for &i in &v[i].1 {
            set.insert(i);
        }
    }
    println!("{}", result);
}
