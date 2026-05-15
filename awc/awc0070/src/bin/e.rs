use proconio::{input, marker::Usize1};

/// 巡回セールスマン問題 (Traveling Salesman Problem) を bit DP で解く
///
/// state: state[bits][i] bits 到達済みの、現在位置 i でのコスト最小値。事前に usize::MAX で埋めておくこと
/// dist: i から j に移動するときのコスト
fn solve_traveling_salesman<F>(state: &mut Vec<Vec<usize>>, dist: F)
where
    F: Fn(usize, usize) -> usize,
{
    let m = state[0].len();
    assert_eq!(1 << m, state.len());

    let bit_test = |bits: usize, i: usize| bits & (1 << i) != 0;

    for visited in 0..(1 << m) {
        for i in 0..m {
            if state[visited][i] == usize::MAX {
                continue;
            }

            for j in 0..m {
                if bit_test(visited, j) {
                    continue;
                }
                let x = state[visited][i] + dist(i, j);
                let visited = visited | (1 << j);
                state[visited][j] = state[visited][j].min(x);
            }
        }
    }
}

fn build_ungraph_weight(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(u, v, w) in uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    graph
}

/// 非負の重み付きグラフの s からすべてのノードへの最短距離をダイクストラ法で解く
fn shortest_all_graph_weight(s: usize, graph: &[Vec<(usize, usize)>]) -> Vec<Option<usize>> {
    use std::{cmp::Reverse, collections::BinaryHeap};

    let n = graph.len();
    let mut v = vec![None; n];

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), s));
    v[s] = Some(0);

    while let Some((Reverse(step), i)) = heap.pop() {
        if step > v[i].unwrap_or(usize::MAX) {
            continue;
        }

        for &(j, w) in &graph[i] {
            let step = step + w;
            let step0 = v[j].unwrap_or(usize::MAX);
            if step < step0 {
                v[j] = Some(step);
                heap.push((Reverse(step), j));
            }
        }
    }
    v
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut v: [Usize1; k],
        abc: [(Usize1, Usize1, usize); m],
    }

    let k = k + 1;
    v.insert(0, 0); // 始点を追加

    let graph = {
        let mut graph = vec![vec![usize::MAX; k]; k];
        let graph0 = build_ungraph_weight(n, &abc);

        for (i, &u) in v.iter().enumerate() {
            let shortest = shortest_all_graph_weight(u, &graph0);
            for (j, &v) in v.iter().enumerate() {
                graph[i][j] = shortest[v].unwrap();
            }
        }
        graph
    };
    // eprintln!("{:?}", &graph);

    let mut state = vec![vec![usize::MAX; k]; 1 << k];
    state[1][0] = 0;
    solve_traveling_salesman(&mut state, |i, j| graph[i][j]);

    let mut result = usize::MAX;
    for j in 1..k {
        let result0 = state[(1 << k) - 1][j].saturating_add(graph[j][0]);
        result = result.min(result0);
    }
    println!("{result}");
}
