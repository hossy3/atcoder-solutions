use pathfinding::directed::dijkstra::dijkstra_all;
use proconio::{input, marker::Usize1};

fn build_ungraph_with_cost(n: usize, uvc: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, cost)
    for &(u, v, c) in uvc {
        graph[u].push((v, c));
        graph[v].push((u, c));
    }
    graph
}

fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, usize)],
    }

    let graph = build_ungraph_with_cost(n, &abc);
    let reachable0 = dijkstra_all(&0, |&i| graph[i].iter().map(|&pair| pair));
    let reachable1 = dijkstra_all(&(n - 1), |&i| graph[i].iter().map(|&pair| pair));

    let mut v = vec![(0usize, 0usize); n];
    for (i, (_, c)) in reachable0 {
        v[i].0 = c;
    }
    for (i, (_, c)) in reachable1 {
        v[i].1 = c;
    }

    for (c0, c1) in v {
        let result = c0 + c1;
        println!("{result}");
    }
}
