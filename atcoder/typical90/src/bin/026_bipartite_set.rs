use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut v = vec![vec![0; 0]; n];
    for &(a, b) in ab.iter() {
        v[a].push(b);
        v[b].push(a);
    }
    let mut s = (BTreeSet::new(), BTreeSet::new());
    let mut stack = vec![(ab[0].0, 0); 1];
    while let Some((a, c)) = stack.pop() {
        let s = if c == 0 { &mut (s.0) } else { &mut (s.1) };
        if s.contains(&a) {
            continue;
        }
        s.insert(a);
        for x in v[a].iter() {
            stack.push((*x, 1 - c));
        }
    }

    let s = if s.0.len() >= s.1.len() { s.0 } else { s.1 };
    let v: Vec<_> = s.iter().collect();
    let result = v[..(n / 2)].iter().map(|&&x| x + 1).join(" ");
    println!("{result}");
}
