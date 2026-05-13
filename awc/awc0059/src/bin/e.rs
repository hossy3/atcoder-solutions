use std::mem::swap;

use proconio::{input, marker::Usize1};

// 親から子を辿る単方向グラフ
fn build_digraph(n: usize, p: &[usize]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for (i, &u) in p.iter().enumerate() {
        let v = i + 1;
        graph[u].push(v);
    }
    graph
}

// Returns Vec<(order, (parent, level))>
fn build_preorder(s: usize, graph: &[Vec<usize>]) -> Vec<(usize, (usize, usize))> {
    let n = graph.len();
    let mut v = vec![(0, (0, 0)); n];
    let mut order = 0usize;
    let mut stack = vec![];
    stack.push((s, (s, 0usize))); // s.parent == s

    while let Some((i, (parent, level))) = stack.pop() {
        v[i] = (order, (parent, level));
        order += 1;
        for &j in &graph[i] {
            stack.push((j, (i, level + 1)));
        }
    }
    v
}

fn build_parents_doubling(s: usize, parents: &[usize]) -> Vec<Vec<usize>> {
    let n = parents.len();
    let nbits = n.ilog2() as usize + 1;
    let mut m = vec![vec![s; nbits]; n];
    for (i, &parent) in parents.iter().enumerate() {
        m[i][0] = parent;
    }
    for i in 0..(nbits - 1) {
        for j in 0..n {
            m[j][i + 1] = m[m[j][i]][i];
        }
    }
    m
}

fn f((x, y): (usize, usize), parents: &[Vec<usize>], levels: &[usize]) -> usize {
    let nbits = parents[0].len();
    let lca = |mut l: usize, mut r: usize| -> usize {
        if levels[l] < levels[r] {
            swap(&mut l, &mut r);
        }
        for i in (0..nbits).rev() {
            if levels[l] - levels[r] >= 1 << i {
                l = parents[l][i];
            }
        }
        if l == r {
            return l;
        }

        for i in (0..nbits).rev() {
            if parents[l][i] != parents[r][i] {
                l = parents[l][i];
                r = parents[r][i];
            }
        }
        parents[l][0]
    };

    lca(x, y) + 1
}

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Usize1; n - 1],
        xy: [(Usize1, Usize1); q],
    }

    let graph = build_digraph(n, &p);
    let preorder = build_preorder(0, &graph);

    let parents: Vec<usize> = preorder.iter().map(|&(_, (parent, _))| parent).collect();
    let parents = build_parents_doubling(0, &parents);

    let levels: Vec<usize> = preorder.iter().map(|&(_, (_, level))| level).collect();

    for xy in xy {
        let result = f(xy, &parents, &levels);
        println!("{result}");
    }
}
