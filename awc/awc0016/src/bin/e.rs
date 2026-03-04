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

fn f(p: &[isize], s: usize, t: usize, graph: &[Vec<(usize, isize)>]) -> isize {
    let mut map = HashMap::new();
    let mut queue = BinaryHeap::new();
    let bits = 1usize << s;
    queue.push((s, p[s], bits));
    let mut result = isize::MIN;
    while let Some((i, score, bits)) = queue.pop() {
        // eprintln!("{}, {}, {:?}", i, score, &set);
        if let Some(&score0) = map.get(&(i, bits)) {
            if score <= score0 {
                continue;
            }
        }
        map.insert((i, bits), score);
        if i == t {
            result = result.max(score);
        }

        for &(j, w) in &graph[i] {
            let score = score - w + if bits.bit_test(j) { 0 } else { p[j] };
            if let Some(&score0) = map.get(&(j, bits)) {
                if score <= score0 {
                    continue;
                }
            }
            let bits = bits | 1 << j;
            queue.push((j, score, bits));
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
