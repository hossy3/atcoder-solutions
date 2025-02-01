use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    let mut results = vec![];
    a.sort_by_key(|&x| Reverse(x));
    b.sort_by_key(|&x| Reverse(x));
    c.sort_by_key(|&x| Reverse(x));

    let f = |i, j, k| a[i] * b[j] + b[j] * c[k] + c[k] * a[i];
    let g = |i, j, k| i * n * n + j * n + k;

    let mut cand = BinaryHeap::new();
    cand.push((
        a[0] * b[0] + b[0] * c[0] + c[0] * a[0],
        0usize,
        0usize,
        0usize,
    ));

    let mut set = HashSet::new();
    while let Some((x, ia, ib, ic)) = cand.pop() {
        results.push(x);
        if results.len() == k {
            break;
        }
        if ia < n - 1 && set.insert(g(ia + 1, ib, ic)) {
            cand.push((f(ia + 1, ib, ic), ia + 1, ib, ic));
        }
        if ib < n - 1 && set.insert(g(ia, ib + 1, ic)) {
            cand.push((f(ia, ib + 1, ic), ia, ib + 1, ic));
        }
        if ic < n - 1 && set.insert(g(ia, ib, ic + 1)) {
            cand.push((f(ia, ib, ic + 1), ia, ib, ic + 1));
        }
    }

    let Some(&result) = results.last() else {
        unreachable!()
    };
    println!("{result}");
}
