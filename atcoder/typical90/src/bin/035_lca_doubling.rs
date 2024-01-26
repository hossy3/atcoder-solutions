use proconio::{input, marker::Usize1};
use std::mem::swap;

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

// Returns Vec<(order, (parent, level))>
fn build_preorder(s: usize, graph: &[Vec<usize>]) -> Vec<(usize, (usize, usize))> {
    let n = graph.len();
    let mut v = vec![(0, (0, 0)); n];
    let mut visited = vec![false; n];
    let mut order = 0usize;
    let mut stack = vec![];
    stack.push((s, (s, 0usize))); // s.parent == s

    while let Some((i, (parent, level))) = stack.pop() {
        v[i] = (order, (parent, level));
        visited[i] = true;
        order += 1;
        for &j in &graph[i] {
            if !visited[j] {
                stack.push((j, (i, level + 1)));
            }
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

fn f(mut v: Vec<usize>, preorder: &[usize], parents: &[Vec<usize>], levels: &[usize]) -> usize {
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

    let distance = |l: usize, r: usize| -> usize {
        let m = lca(l, r);
        (levels[l] - levels[m]) + (levels[r] - levels[m])
    };

    v.sort_by_key(|&k| preorder[k]);

    let mut result = 0;
    for v in v.windows(2) {
        result += distance(v[0], v[1]);
    }
    result += distance(v[0], v[v.len() - 1]);

    result / 2
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        q: usize,
    }

    let graph = build_ungraph(n, &ab);
    let preorder = build_preorder(0, &graph);

    let parents: Vec<usize> = preorder.iter().map(|&(_, (parent, _))| parent).collect();
    let parents = build_parents_doubling(0, &parents);

    let levels: Vec<usize> = preorder.iter().map(|&(_, (_, level))| level).collect();
    let preorder: Vec<usize> = preorder.iter().map(|&(order, (_, _))| order).collect();

    for _ in 0..q {
        input! {
            k: usize,
            v: [Usize1; k],
        }
        let result = f(v, &preorder, &parents, &levels);
        println!("{result}");
    }
}
