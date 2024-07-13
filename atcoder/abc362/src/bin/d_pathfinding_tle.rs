use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        uvb: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n]; // node, (edge, cost)
    for (u, v, b) in uvb {
        graph[u].push((v, a[v] + b));
        graph[v].push((u, a[u] + b));
    }
    let reachable = dijkstra_all(&0, |&i| graph[i].iter().copied());
    let result = (1..n)
        .map(|i| a[0] + reachable.get(&i).and_then(|&(_, c)| Some(c)).unwrap_or(0))
        .join(" ");
    println!("{result}");
}
