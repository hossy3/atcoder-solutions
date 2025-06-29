use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let set = BTreeSet::from_iter(a.iter());
    println!("{}", set.len());
    println!("{}", set.iter().join(" "));
}
