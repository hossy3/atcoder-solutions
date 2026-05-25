use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use ac_library::{Dsu, Min, Segtree};
use proconio::{input, marker::Usize1};

// Maximum Spanning Tree

fn euler_tour_impl(
    graph: &[Vec<usize>],
    i: usize,
    parent: usize,
    level: usize,
    v: &mut Vec<(usize, (usize, usize))>,
) {
    v.push((i, (parent, level)));
    for &j in graph[i].iter().rev() {
        if j != parent {
            euler_tour_impl(graph, j, i, level + 1, v);
            v.push((i, (parent, level)));
        }
    }
}

// Returns Vec<(node, (parent, level))>
fn euler_tour(s: usize, graph: &[Vec<usize>]) -> Vec<(usize, (usize, usize))> {
    let mut v = Vec::with_capacity(graph.len() * 2 - 1);
    euler_tour_impl(graph, s, s, 0, &mut v);
    v
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    // MST と代替辺候補に分ける
    let mut heap = BinaryHeap::new();
    for (u, v, w) in uvw {
        heap.push((Reverse(w), u, v));
    }

    let mut mst = vec![vec![]; n];
    let mut heap0 = BinaryHeap::new(); // mst 以外を

    let mut dsu = Dsu::new(n);
    let mut map = HashMap::new();
    let mut len = 0;
    while let Some((Reverse(w), u, v)) = heap.pop() {
        if dsu.same(u, v) {
            heap0.push((Reverse(w), u, v));
        } else {
            dsu.merge(u, v);
            len += w;
            mst[u].push(v);
            mst[v].push(u);
            if u < v {
                map.insert((u, v), (w, usize::MAX)); // (MST の辺の重み, 代替辺の重み)
            } else {
                map.insert((v, u), (w, usize::MAX));
            }
        }
    }

    // 最小共通祖先 (LCA) を高速に求められるように準備する
    let paths = euler_tour(0, &mst);

    let mut segtree = Segtree::<Min<usize>>::new(paths.len());
    for (i, &(x, (_, level))) in paths.iter().enumerate() {
        let value = (level << 32) | x;
        segtree.set(i, value);
    }

    let mut preorder = vec![0; n];
    for (i, &(x, _)) in paths.iter().enumerate().rev() {
        preorder[x] = i;
    }

    // 代替辺のない最初の先祖を高速に求められるように準備する
    let mut parents = vec![usize::MAX; n];
    for &(node, (parent, level)) in &paths {
        if level > 0 {
            parents[node] = parent;
        }
    }
    let mut tops = (0..n).collect::<Vec<_>>();
    let mut dsu = Dsu::new(n);

    // 重みの小さな辺から代替辺として採用していく
    while let Some((Reverse(w), u, v)) = heap0.pop() {
        let f = |x: usize| ((x >> 32), x & ((1 << 32) - 1));
        let (l, r) = (preorder[u], preorder[v]);
        let (l, r) = if l < r { (l, r) } else { (r, l) };

        let (_, x_lca) = f(segtree.prod(l..=r));
        for i in 0..2 {
            let u = if i == 0 { u } else { v };
            let mut top = tops[dsu.leader(u)];
            while !dsu.same(top, x_lca) {
                let parent = parents[top];
                let (u, v) = if parent < top {
                    (parent, top)
                } else {
                    (top, parent)
                };
                if let Some(w0) = map.get_mut(&(u, v)) {
                    w0.1 = w;
                }
                let top0 = tops[dsu.leader(parent)];
                let x0 = dsu.merge(parent, top);
                tops[x0] = top0;
                top = top0;
            }
        }
    }

    // 代替辺にすると最も重みが増えるところを返す
    let result = len + map.iter().map(|(_, &(w, w0))| w0 - w).max().unwrap();
    println!("{result}");
}
