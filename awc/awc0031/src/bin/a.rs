use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        _m: usize,
        k: usize,
    }

    let mut vit = vec![];
    for i in 0..n {
        input! {
            v: usize,
            c: usize,
            t: [usize; c],
        }
        vit.push((Reverse(v), i, t));
    }

    vit.sort_unstable();
    let t = vit[..k]
        .iter()
        .map(|(_, _, t)| t.clone())
        .collect::<Vec<_>>();
    let mut set = t[0].iter().collect::<std::collections::HashSet<_>>();
    for t in t.iter().skip(1) {
        set.retain(|&x| t.contains(&x));
    }

    let result = set.len();
    println!("{result}");
}
