use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn f(n: usize, uvw: &[(usize, usize, isize)], e: &[usize]) -> isize {
    let mut dsu = Dsu::new(n);
    let mut result = 0;
    let mut set = HashSet::new();
    for &e in e {
        let (u, v, w) = uvw[e];
        if dsu.same(u, v) {
            return -1; // ループがあると全域木にならない
        }
        dsu.merge(u, v);
        result += w;
        set.insert(e);
    }

    let mut heap = BinaryHeap::new();
    for (i, &(u, v, w)) in uvw.iter().enumerate() {
        if !set.contains(&i) {
            heap.push((Reverse(w), u, v));
        }
    }

    while let Some((Reverse(w), u, v)) = heap.pop() {
        if dsu.same(u, v) {
            continue;
        }
        dsu.merge(u, v);
        result += w;
    }

    if dsu.groups().len() > 1 {
        return -1; // 複数のグループに分かれているので全域木にならない
    }

    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvw: [(Usize1, Usize1, isize); m],
        e: [Usize1; k],
    }
    let result = f(n, &uvw, &e);
    println!("{result}");
    // let yes = true;
    // println!("{}", if yes { "Yes" } else { "No" });
}
