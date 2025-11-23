use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize, usize); q],
    }
    let mut s = BTreeSet::new();
    s.insert(0);
    s.insert(l);
    for &(c, x) in &cx {
        if c == 1 {
            s.insert(x);
        } else {
            let x0 = s.range(..=x).last().unwrap();
            let x1 = s.range(x..).next().unwrap();
            println!("{}", x1 - x0);
        }
    }
}
