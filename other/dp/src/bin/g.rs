use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
    }
    graph
}

fn tree_dp(n: usize, xy: &[(usize, usize)]) -> usize {
    let graph_xy = build_digraph(n, xy);
    let graph_yx = build_digraph(n, &xy.iter().map(|&(x, y)| (y, x)).collect::<Vec<_>>());

    let mut results = vec![0usize; n];
    let mut counts = vec![0usize; n];
    let mut stack = vec![];
    for (i, parents) in graph_xy.iter().enumerate() {
        counts[i] = parents.len();
        if parents.is_empty() {
            stack.push(i);
        }
    }
    while let Some(i) = stack.pop() {
        for &j in &graph_yx[i] {
            results[j] = results[j].max(results[i] + 1);
            counts[j] -= 1;
            if counts[j] == 0 {
                stack.push(j);
            }
        }
    }
    *results.iter().max().unwrap()
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }
    let result = tree_dp(n, &xy);
    println!("{result}");
}
