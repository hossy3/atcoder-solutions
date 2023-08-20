use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn f(v: &Vec<Vec<usize>>, i: usize, set: &BTreeSet<(usize, usize)>, n: usize, t: usize) -> usize {
    if i == n {
        return if v.len() == t { 1 } else { 0 };
    }
    let mut result = 0;
    'outer: for i0 in 0..v.len() {
        for &x in &v[i0] {
            if set.contains(&(i, x)) {
                continue 'outer;
            }
        }
        let mut v0 = v.clone();
        v0[i0].push(i);
        result += f(&v0, i + 1, set, n, t);
    }

    if v.len() < t {
        let mut v0 = v.clone();
        v0.push(vec![i]);
        result += f(&v0, i + 1, set, n, t);
    }
    result
}

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut set = BTreeSet::new();
    for &(a, b) in &ab {
        set.insert((a, b));
        set.insert((b, a));
    }

    let result = f(&vec![], 0, &set, n, t);

    println!("{}", result);
}
