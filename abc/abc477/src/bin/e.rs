use proconio::{input, marker::Usize1};

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
        q: usize,
        a: [usize; n],
        b: [usize; n],
        st: [(Usize1, Usize1); q],
    }

    // A だけを通る距離 = 累積和 を2周分求める
    let mut sum = vec![0usize; n * 2];
    for i in 0..(2 * n - 1) {
        sum[i + 1] = sum[i] + a[i % n];
    }
    // eprintln!("{:?}", &sum);

    // B を始点とした最短経路を求める
    let mut graph = vec![vec![]; n + 1];
    for i in 0..n {
        let j = (i + 1) % n;
        graph[i].push((j, a[i]));
        graph[j].push((i, a[i]));
    }
    for i in 0..n {
        graph[n].push((i, b[i]));
        graph[i].push((n, b[i]));
    }
    let shortest = shortest_all_graph_weight(n, &graph);
    // eprintln!("{:?}", &shortest);

    for (s, t) in st {
        let result = if t == n {
            shortest[s].unwrap()
        } else {
            let dist0 = sum[t] - sum[s];
            let dist1 = sum[s + n] - sum[t];
            let dist2 = shortest[s].unwrap() + shortest[t].unwrap();
            dist0.min(dist1).min(dist2)
        };
        println!("{result}");
    }
}
