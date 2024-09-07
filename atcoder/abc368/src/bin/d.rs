use std::collections::{BTreeSet, VecDeque};

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

// Returns Vec<(step, parent)>
fn dijkstra_all(s: usize, graph: &[Vec<usize>]) -> Vec<Option<(usize, usize)>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    queue.push_back((0usize, s));

    while let Some((step, i)) = queue.pop_front() {
        for &j in &graph[i] {
            if j == s || v[j].is_some() {
                continue;
            }
            v[j] = Some((step + 1, i));
            queue.push_back((step + 1, j));
        }
    }
    v
}

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(Usize1, Usize1); n - 1],
        v: [Usize1; k],
    }

    let graph = build_ungraph(n, &ab);
    let tree = dijkstra_all(0, &graph);

    let mut visited = vec![false; n];
    let mut set: BTreeSet<_> = v.iter().cloned().collect();
    while let Some(i) = set.pop_first() {
        visited[i] = true;
        if let Some((_, j)) = tree[i] {
            if !visited[j] {
                set.insert(j);
            }
        }
    }

    let set: BTreeSet<_> = v.iter().cloned().collect();
    let mut i = 0;
    while !set.contains(&i) {
        if graph[i].iter().filter(|&&i| visited[i]).count() > 1 {
            break;
        }
        visited[i] = false;
        i = *graph[i].iter().find(|&&i| visited[i]).unwrap();
    }

    let result = visited.iter().filter(|&&b| b).count();
    println!("{result}");
}
