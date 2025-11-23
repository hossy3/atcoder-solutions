use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        w: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let graph = build_ungraph(n, &uv);
    let mut nodes = vec![BTreeSet::new(); n];
    let mut queue = BinaryHeap::new();
    queue.push((std::cmp::Reverse(0), 0, 0)); // weight, current score, node
    while let Some((Reverse(weight), score, i)) = queue.pop() {
        let score = score + weight;
        let weight = weight + w[i];
        if nodes[i].len() > 0 {
            if let Some(&Reverse((_, score0))) = nodes[i].range(Reverse((weight, score))..).next() {
                if score0 <= score {
                    continue; // より軽くより消費燃料が少ない場合があるのでスキップする
                }
            }
        }

        while let Some(&Reverse((w0, score0))) = nodes[i].range(..=Reverse((weight, score))).last()
        {
            if score0 >= score {
                nodes[i].remove(&Reverse((w0, score0))); // より重くより消費燃料が多い場合は削除する
            } else {
                break;
            }
        }
        nodes[i].insert(Reverse((weight, score)));

        for &node in &graph[i] {
            queue.push((Reverse(weight), score, node));
        }
    }

    for node in &nodes {
        let Some(Reverse((_, score))) = node.first() else {
            unreachable!()
        };
        println!("{score}");
    }
}
