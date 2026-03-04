use std::collections::{BinaryHeap, HashMap};

use fixedbitset::FixedBitSet;
use proconio::{input, marker::Usize1};

fn f(p: &[isize], s: usize, t: usize, graph: &[Vec<(usize, isize)>]) -> isize {
    let n = p.len();
    let mut map = HashMap::new();
    let mut queue = BinaryHeap::new();
    let mut set = FixedBitSet::with_capacity(n);
    set.insert(s);
    queue.push((s, p[s], set));
    let mut result = isize::MIN;
    while let Some((i, score, set)) = queue.pop() {
        // eprintln!("{}, {}, {:?}", i, score, &set);
        if let Some(&score0) = map.get(&(i, set.clone())) {
            if score <= score0 {
                continue;
            }
        }
        map.insert((i, set.clone()), score);
        if i == t {
            result = result.max(score);
        }

        for &(j, w) in &graph[i] {
            let score = score - w + if set.contains(j) { 0 } else { p[j] };
            if let Some(&score0) = map.get(&(j, set.clone())) {
                if score <= score0 {
                    continue;
                }
            }
            let mut set = set.clone();
            set.insert(j);
            queue.push((j, score, set));
        }
    }

    result
}

fn build_ungraph_weight(n: usize, uvw: &[(usize, usize, isize)]) -> Vec<Vec<(usize, isize)>> {
    let mut graph = vec![vec![]; n]; // node, (edge, weight)
    for &(u, v, w) in uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [isize; n],
        s: Usize1,
        t: Usize1,
        uvw: [(Usize1, Usize1, isize); m]
    }

    let graph = build_ungraph_weight(n, &uvw);
    let result = f(&p, s, t, &graph);
    println!("{result}");
}
