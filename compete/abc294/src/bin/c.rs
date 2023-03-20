use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut c = a.clone();
    c.append(&mut b.clone());
    c.sort();

    let mut m = BTreeMap::new();
    for (i, &x) in c.iter().enumerate() {
        m.insert(x, i + 1);
    }

    let result = a.iter().map(|x| m.get(x).unwrap()).join(" ");
    println!("{}", result);

    let result = b.iter().map(|x| m.get(x).unwrap()).join(" ");
    println!("{}", result);
}
