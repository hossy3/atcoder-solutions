use std::collections::BTreeMap;

use proconio::input;

// coordinate compression

fn main() {
    input! {
        _: usize,
        _: usize,
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut mr = BTreeMap::new();
    let mut mc = BTreeMap::new();
    for &(a, b) in &ab {
        mr.insert(a, 0usize);
        mc.insert(b, 0usize);
    }
    for (i, r) in mr.iter_mut().enumerate() {
        *r.1 = i + 1;
    }
    for (i, c) in mc.iter_mut().enumerate() {
        *c.1 = i + 1;
    }
    for &(a, b) in &ab {
        let (a, b) = (mr.get(&a).unwrap(), mc.get(&b).unwrap());
        println!("{} {}", a, b);
    }
}
