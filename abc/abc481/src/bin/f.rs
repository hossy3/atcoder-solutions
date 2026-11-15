use std::collections::{BTreeSet, VecDeque};

use proconio::{input, marker::Usize1};

fn build_digraph_weight(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(u, v, w) in uvw {
        graph[u].push((v, w));
    }
    graph
}

fn shortest_all_ungraph(s: usize, graph: &[Vec<(usize, usize)>]) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    v[s] = Some(0usize);
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        for &(j, _) in &graph[i] {
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
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let graph = build_digraph_weight(n, &abc);

    // Phase 1: 最短距離を求める
    let steps = shortest_all_ungraph(0, &graph);

    // Phase 2: 最短距離の範囲内で一番小さな移動方法を調べる
    const K: usize = 998244353;
    let max_step = steps.iter().map(|&x| x.unwrap()).max().unwrap();
    let mut v0 = vec![vec![]; max_step + 1];
    for (i, step) in steps.iter().enumerate() {
        v0[step.unwrap()].push(i);
    }

    let mut v = vec![None; n];
    v[0] = Some((0usize, 0usize)); // 大小関係 0番目, スコア 0
    for i in 0..max_step {
        for &a in &v0[i] {
            for &(b, c) in &graph[a] {
                if steps[b] != Some(i + 1) {
                    continue;
                }

                let order = v[a].unwrap().0 * 10 + c;
                let score = (v[a].unwrap().1 * 10 + c) % K;

                if let Some((order0, _)) = v[b] {
                    if order0 > order {
                        v[b] = Some((order, score));
                    }
                } else {
                    v[b] = Some((order, score));
                }

                if let Some((order0, _)) = v[b] {
                    if order0 > order {
                        v[b] = Some((order, score));
                    }
                } else {
                    v[b] = Some((order, score));
                }
            }
        }

        // 次のステップのために大小関係を正規化する
        let mut set = BTreeSet::new();
        for &a in &v0[i + 1] {
            let order = v[a].unwrap().0;
            set.insert(order);
        }

        let array: Vec<_> = set.iter().cloned().collect();
        for &a in &v0[i + 1] {
            let (order, score) = v[a].unwrap();
            let order0 = array.partition_point(|&x| x < order);
            v[a] = Some((order0, score));
        }
    }

    for &x in &v[1..] {
        let Some((_, score)) = x else { unreachable!() };
        println!("{score}");
    }
}
