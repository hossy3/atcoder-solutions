use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        cp: [(Usize1, Usize1); q],
    }

    let mut roots = vec![true; n];
    let mut prev = vec![usize::MAX; n];
    let mut next = vec![usize::MAX; n];
    for &(c, p) in &cp {
        if prev[c] != usize::MAX {
            next[prev[c]] = usize::MAX;
        }
        next[p] = c;
        prev[c] = p;
        roots[c] = false;
        // eprintln!("{c} {p}");
        // eprintln!("{:?}", &next);
        // eprintln!("{:?}", &prev);
    }

    let mut results = vec![0usize; n];
    for i in 0..n {
        if !roots[i] {
            continue;
        }
        let mut result = 0;
        let mut cur = i;
        while cur != usize::MAX {
            result += 1;
            cur = next[cur];
        }
        results[i] = result;
    }
    println!("{}", results.iter().join(" "));
}
