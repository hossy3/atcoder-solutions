use std::collections::{BinaryHeap, HashMap};

use proconio::{input, marker::Usize1};

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
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
fn shortest_graph(
    graph: &[Vec<(usize, usize)>],
    f: usize,
    r: &[usize],
    s: usize,
    e: usize,
) -> Option<usize> {
    let mut map = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push((f + r[s], 1 << s, s));
    let mut cand = None;

    while let Some((hp, bits, i)) = heap.pop() {
        if i == e {
            if cand.unwrap_or(0) <= hp {
                cand = Some(hp);
            }
        }

        if let Some(&hp0) = map.get(&(bits, i)) {
            if hp0 >= hp {
                continue;
            }
        }
        map.insert((bits, i), hp);
        for &(j, w) in &graph[i] {
            let (mut bits, mut hp) = (bits, hp);
            if hp < w {
                continue; // 体力が足りないので通れない
            }
            hp -= w;
            if !bits.bit_test(j) {
                bits |= 1 << j;
                hp += r[j];
            }
            if let Some(&hp0) = map.get(&(bits, j)) {
                if hp0 >= hp {
                    continue;
                }
            }
            heap.push((hp, bits, j));
        }
    }
    cand
}

fn main() {
    input! {
        n: usize,
        m: usize,
        f: usize,
        r: [usize; n],
        uvw: [(Usize1, Usize1, usize); m],
    }

    let graph = build_ungraph_weight(n, &uvw);
    if let Some(result) = shortest_graph(&graph, f, &r, 0, n - 1) {
        println!("{result}");
    } else {
        println!("-1");
    }
}
