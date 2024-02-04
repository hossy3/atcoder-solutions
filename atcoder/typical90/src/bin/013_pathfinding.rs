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
    let reachable0 = dijkstra_all(&0, |&i| graph[i].iter().copied());
    let reachable1 = dijkstra_all(&(n - 1), |&i| graph[i].iter().copied());

    for i in 0..n {
        let c0 = reachable0.get(&i).and_then(|&(_, c)| Some(c)).unwrap_or(0);
        let c1 = reachable1.get(&i).and_then(|&(_, c)| Some(c)).unwrap_or(0);
        let result = c0 + c1;
        println!("{result}");
    }
}
