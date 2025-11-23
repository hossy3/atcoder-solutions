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

    let mut heap = BinaryHeap::new();
    a.sort_by_key(|&x| Reverse(x));
    b.sort_by_key(|&x| Reverse(x));
    c.sort_by_key(|&x| Reverse(x));

    let mut cand = BinaryHeap::new();
    cand.push((
        a[0] * b[0] + b[0] * c[0] + c[0] * a[0],
        0usize,
        0usize,
        0usize,
    ));

    let mut set = HashSet::new();
    while let Some((x, ia, ib, ic)) = cand.pop() {
        if heap.len() >= k {
            let Some(&Reverse(y)) = heap.peek() else {
                unreachable!()
            };
            if x < y {
                continue;
            }
            heap.pop();
        }
        heap.push(Reverse(x));
        if ia < n - 1 && set.insert((ia + 1) * n * n + ib * n + ic) {
            cand.push((
                a[ia + 1] * b[ib] + b[ib] * c[ic] + c[ic] * a[ia + 1],
                ia + 1,
                ib,
                ic,
            ));
        }
        if ib < n - 1 && set.insert(ia * n * n + (ib + 1) * n + ic) {
            cand.push((
                a[ia] * b[ib + 1] + b[ib + 1] * c[ic] + c[ic] * a[ia],
                ia,
                ib + 1,
                ic,
            ));
        }
        if ic < n - 1 && set.insert(ia * n * n + ib * n + (ic + 1)) {
            cand.push((
                a[ia] * b[ib] + b[ib] * c[ic + 1] + c[ic + 1] * a[ia],
                ia,
                ib,
                ic + 1,
            ));
        }
    }

    let Some(Reverse(result)) = heap.pop() else {
        unreachable!()
    };
    println!("{result}");
}
