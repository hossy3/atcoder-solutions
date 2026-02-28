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

fn main() {
    input! {
        n: usize,
        u: Usize1,
        v: Usize1,
        ab: [(Usize1, Usize1); n - 1],
    }

    let graph = build_ungraph(n, &ab);

    let shortest_u = shortest_all_ungraph(u, &graph); // 高橋君の歩数
    let shortest_v = shortest_all_ungraph(v, &graph); // 青木君の歩数

    let mut result = 0;
    for i in 0..n {
        let Some(u0) = shortest_u[i] else {
            unreachable!()
        };
        let Some(v0) = shortest_v[i] else {
            unreachable!()
        };
        if u0 < v0 {
            result = result.max(v0 - 1); // 最後は枝の末端の一つ手前になるはず
        }
    }
    println!("{result}");
}
