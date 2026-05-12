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

/// 非負の重み付きグラフの s から e までの最短距離をダイクストラ法で解く
fn shortest_graph_weight(graph: &[Vec<(usize, usize)>], s: usize, e: usize) -> Option<usize> {
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
        if i == e {
            return Some(step);
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

    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
        s: Usize1,
        k: usize,
        d: [Usize1; k],
    }

    let mut d0 = vec![s];
    d0.append(&mut d.clone());
    let n0 = d0.len();

    let mut m = vec![vec![0; n0]; n0];
    let graph = build_ungraph_weight(n, &uvw);
    for i in 0..n0 {
        for j in i..n0 {
            let x = shortest_graph_weight(&graph, d0[i], d0[j]).unwrap();
            m[i][j] = x;
            m[j][i] = x;
        }
    }

    let mut state = vec![vec![usize::MAX; n0]; 1 << n0];
    state[1][0] = 0;
    solve_traveling_salesman(&mut state, |i, j| m[i][j]);
    let result = (1..n0)
        .map(|i| state[(1 << n0) - 1][i] + m[i][0])
        .min()
        .unwrap();
    println!("{result}");
}
