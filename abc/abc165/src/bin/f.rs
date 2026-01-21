use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn shortest_all_ungraph(s: usize, graph: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    v[s] = Some(0usize);
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        for &j in &graph[i] {
            if v[j] == None {
                v[j] = Some(step + 1);
                queue.push_back((step + 1, j));
            }
        }
    }
    v
}

fn compress_coordinates(a: &[usize]) -> Vec<usize> {
    let mut sorted = a.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut map = std::collections::HashMap::new();
    for (i, &x) in sorted.iter().enumerate() {
        map.insert(x, i);
    }
    a.iter().map(|&x| map[&x]).collect()
}

fn dfs(
    graph: &[Vec<usize>],
    shortest: &[usize],
    a: &[usize],
    i: usize,
    results: &mut [usize],
    lis: &mut Vec<usize>,
) {
    let x = a[i];
    let j = lis.partition_point(|&y| y < x);
    let mut pushed = false; // undo 用
    let mut updated = None; // undo 用

    if j == lis.len() {
        pushed = true;
        lis.push(x);
    } else if lis[j] > x {
        updated = Some(lis[j]);
        lis[j] = x;
    }
    results[i] = lis.len();

    for &j in &graph[i] {
        if shortest[i] < shortest[j] {
            dfs(graph, shortest, a, j, results, lis);
        }
    }

    if pushed {
        lis.pop();
    } else if let Some(x) = updated {
        lis[j] = x;
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        uv: [(Usize1, Usize1); n - 1],
    }
    let graph = build_ungraph(n, &uv);
    let shortest: Vec<_> = shortest_all_ungraph(0, &graph)
        .iter()
        .map(|&x| x.unwrap())
        .collect();
    let a = compress_coordinates(&a);

    let mut results = vec![usize::MAX; n];
    let mut lis = vec![];
    dfs(&graph, &shortest, &a, 0, &mut results, &mut lis);

    for result in results {
        println!("{result}");
    }
}
