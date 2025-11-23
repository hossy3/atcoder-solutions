use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
    }

    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            p: usize,
            c: usize,
            mut f: [usize; c],
        }
        let mut set = BTreeSet::new();
        for &x in &f {
            set.insert(x);
        }
        v.push((p, set));
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if v[i].0 < v[j].0 {
                continue;
            }
            if !v[i].1.is_subset(&v[j].1) {
                continue;
            }
            if v[i].0 == v[j].0 && v[i].1 == v[j].1 {
                continue;
            }
            println!("Yes");
            return;
        }
    }

    println!("No");
}
