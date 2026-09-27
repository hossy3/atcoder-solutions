use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn f1(ab: &[(usize, usize)], a0: usize, b0: usize, results: &mut HashSet<(usize, usize)>) {
    for &(a, b) in ab {
        if !(a == a0 || b == a0 || a == b0 || b == b0) {
            return;
        }
    }
    results.insert((a0, b0));
}

fn f0(n: usize, ab: &[(usize, usize)], a0: usize, results: &mut HashSet<(usize, usize)>) {
    for (i, &(a, b)) in ab.iter().enumerate() {
        if !(a == a0 || b == a0) {
            f1(&ab[i..], a0.min(a), a0.max(a), results);
            f1(&ab[i..], a0.min(b), a0.max(b), results);
            return;
        }
    }

    // すべてが候補
    for b in 0..n {
        if a0 != b {
            results.insert((a0.min(b), a0.max(b)));
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut results = HashSet::new();
    f0(n, &ab, ab[0].0, &mut results);
    f0(n, &ab, ab[0].1, &mut results);

    let result = results.len();
    println!("{result}");
}
