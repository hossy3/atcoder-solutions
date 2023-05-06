use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }
    let mut set = BTreeSet::new();
    for &p in &p {
        set.insert(p);
        if set.len() == k + 1 {
            let x = *set.iter().next().unwrap();
            set.remove(&x);
        }
        if set.len() == k {
            let result = set.iter().next().unwrap();
            println!("{}", result);
        }
    }
}
