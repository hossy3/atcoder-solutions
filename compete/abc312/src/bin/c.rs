use std::collections::BTreeSet;

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.sort();
    b.sort();

    let mut set = BTreeSet::new();
    for &a in &a {
        set.insert(a);
    }
    for &b in &b {
        set.insert(b + 1);
    }

    for &x in &set {
        let a0 = a.upper_bound(&x);
        let b0 = b.lower_bound(&x);
        if a0 >= m - b0 {
            println!("{}", x);
            return;
        }
    }
}
