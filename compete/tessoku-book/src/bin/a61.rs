use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1)],
    }

    let mut v = vec![vec![]; n];
    for &(a, b) in &ab {
        v[a].push(b);
        v[b].push(a);
    }
    for i in 0..n {
        v[i].sort();
        let result = v[i].iter().map(|x| x + 1).join(", ");
        println!("{}: {{{}}}", i + 1, result);
    }
}
