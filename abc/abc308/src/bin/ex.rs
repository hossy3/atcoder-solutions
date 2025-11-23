use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, abc: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (adjacent_node, cost)
    for &(a, b, c) in abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }
    for i in 0..n {
        graph[i].sort_by_key(|x| x.1);
    }
    graph
}

fn find_shortest_loop(start: usize, q: usize, graph: &[Vec<(usize, usize)>]) -> usize {
    let mut queue = BinaryHeap::new();
    let mut v = vec![(0, start); graph.len()]; // total cost, second node
    for &(i, c) in &graph[start] {
        queue.push((Reverse(c), i, i)); // cost, node, second node
    }
    while let Some((Reverse(c), i, second)) = queue.pop() {
        if i == start || i == q {
            continue; // not loop
        }
        if v[i].0 > 0 {
            if v[i].1 == second {
                continue;
            }
            return v[i].0 + c; // loop
        }
        v[i] = (c, second);
        for &(i0, c0) in &graph[i] {
            queue.push((Reverse(c + c0), i0, second));
        }
    }

    std::usize::MAX
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }
    let graph = build_ungraph(n, &abc);

    let mut result = std::usize::MAX;
    for i in 0..n {
        if graph[i].len() >= 3 {
            for j in 0..3 {
                let (q, c) = graph[i][j];
                let shortest = find_shortest_loop(i, q, &graph);
                if shortest != std::usize::MAX {
                    result = result.min(shortest + c);
                }
            }
        }
    }

    if result == std::usize::MAX {
        println!("{}", -1);
    } else {
        println!("{}", result);
    }
}
