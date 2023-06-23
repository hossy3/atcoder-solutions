use std::{collections::BTreeMap, ops::Bound};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; m],
        d: [usize; m],
    }
    let mut v = Vec::with_capacity(n + m); // width, type, height
    for i in 0..n {
        v.push((a[i], false, b[i]));
    }
    for i in 0..m {
        v.push((c[i], true, d[i]));
    }
    v.sort();
    v.reverse();

    let mut map = BTreeMap::new();
    for &(_, t, h) in &v {
        if t {
            *map.entry(h).or_insert(0usize) += 1;
        } else {
            if let Some((&h, &count)) = map.range((Bound::Included(h), Bound::Unbounded)).next() {
                if count > 1 {
                    map.insert(h, count - 1);
                } else {
                    map.remove(&h);
                }
            } else {
                println!("{}", "No");
                return;
            }
        }
    }

    println!("{}", "Yes");
}
