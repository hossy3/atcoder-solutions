use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut set = BTreeSet::new();

    for (i, &x) in a.iter().enumerate() {
        if !set.contains(&(i + 1)) {
            set.insert(x);
        }
    }
    let len = n - set.len();
    let mut v = vec![];
    for i in 1..=n {
        if !set.contains(&i) {
            v.push(i);
        }
    }
    let result = v.iter().join(" ");
    println!("{}", len);
    println!("{}", result);
}
