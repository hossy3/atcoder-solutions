use ac_library::{Min, Segtree};
use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

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

fn f(mut v: Vec<usize>, preorder: &[usize], segtree: &Segtree<Min<usize>>) -> usize {
    let distance = |l: usize, r: usize| -> usize {
        let lca_level = segtree.prod(l..=r);
        (segtree.get(l) - lca_level) + (segtree.get(r) - lca_level)
    };

    v.sort_by_key(|&k| preorder[k]);

    let mut result = 0;
    for v in v.windows(2) {
        result += distance(preorder[v[0]], preorder[v[1]]);
    }
    result += distance(preorder[v[0]], preorder[v[v.len() - 1]]);

    result / 2
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        q: usize,
    }

    let graph = build_ungraph(n, &ab);
    let paths = euler_tour(0, &graph);

    let mut segtree = Segtree::<Min<usize>>::new(paths.len() * 2);
    for (i, &(_, (_, level))) in paths.iter().enumerate() {
        segtree.set(i, level);
    }

    let mut preorder = vec![0; n];
    for (i, &(x, _)) in paths.iter().enumerate().rev() {
        preorder[x] = i;
    }

    for _ in 0..q {
        input! {
            k: usize,
            v: [Usize1; k],
        }
        let result = f(v, &preorder, &segtree);
        println!("{result}");
    }
}
