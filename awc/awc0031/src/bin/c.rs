use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

/// 重みなしグラフの s から e までの最短距離を 01-bfs で解く
fn shortest_graph(graph: &[Vec<usize>], s: usize, e: usize) -> Option<usize> {
    let mut set = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        if i == e {
            return Some(step);
        }
        for &j in &graph[i] {
            if set.insert(j) {
                queue.push_back((step + 1, j));
            }
        }
    }
    None
}

fn main() {
    input! {
        n: usize,
        d: isize,
        s: Usize1,
        t: Usize1,
        xy: [(isize, isize); n],
    }

    let mut graph = vec![vec![]; n];
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if (xy[i].0 - xy[j].0).pow(2) + (xy[i].1 - xy[j].1).pow(2) <= d.pow(2) {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    if let Some(result) = shortest_graph(&graph, s, t) {
        println!("{result}");
    } else {
        println!("-1");
    }
}
