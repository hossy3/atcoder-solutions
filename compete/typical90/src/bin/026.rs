use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let mut v = vec![vec![0; 0]; n];
    for &(a, b) in ab.iter() {
        v[a - 1].push(b);
        v[b - 1].push(a);
    }
    let mut s = (BTreeSet::new(), BTreeSet::new());
    let mut stack = vec![(ab[0].0, 0); 1];
    while let Some((a, c)) = stack.pop() {
        let s = if c == 0 { &mut (s.0) } else { &mut (s.1) };
        if s.contains(&a) {
            continue;
        }
        s.insert(a);
        for x in v[a - 1].iter() {
            stack.push((*x, 1 - c));
        }
    }

    let s = if s.0.len() >= s.1.len() { s.0 } else { s.1 };
    let mut v: Vec<usize> = s.into_iter().collect();
    v.resize(n / 2, 0); // 0 is unused
    for x in v {
        print!("{} ", x);
    }
}
