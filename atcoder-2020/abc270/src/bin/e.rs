use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [usize; n],
    }

    let mut m = BTreeMap::new();
    for (i, &x) in a.iter().enumerate() {
        m.insert(i, x);
    }
    while k >= m.len() && k > 0 {
        let len = k / m.len();
        let mut s = BTreeSet::new();
        for (&i, x) in m.iter_mut() {
            let len = len.min(*x);
            *x -= len;
            k -= len;
            if *x == 0 {
                s.insert(i);
            }
        }
        for i in s {
            m.remove(&i);
        }
    }
    for (_, x) in m.iter_mut() {
        if k == 0 {
            break;
        }
        *x -= 1;
        k -= 1;
    }
    for (i, _) in a.iter().enumerate() {
        let x = m.get(&i).unwrap_or(&0);
        print!("{} ", x);
    }
    println!();
}
