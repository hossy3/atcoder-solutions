use std::{cmp::Reverse, collections::BinaryHeap};

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
        uv: [(Usize1, Usize1); m],
        k: usize,
        pd: [(Usize1, usize); k],
    }

    let graph = build_ungraph(n, &uv);

    let mut white = vec![false; n]; // must white
    let mut v = vec![vec![]; n];

    for &(p, d) in &pd {
        let mut v0 = vec![false; n];
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), p));
        while let Some((Reverse(step), i)) = queue.pop() {
            if v0[i] {
                continue;
            }
            v0[i] = true;
            if step == d {
                v[p].push(i); // black candidate
                continue;
            }
            white[i] = true;

            for &j in &graph[i] {
                if v0[j] {
                    continue;
                }
                queue.push((Reverse(step + 1), j));
            }
        }
    }

    for &(p, _) in &pd {
        if v[p].iter().filter(|&&i| !white[i]).count() == 0 {
            println!("{}", "No");
            return;
        }
    }

    println!("{}", "Yes");
    for &x in &white {
        print!("{}", if x { 0 } else { 1 });
    }
    println!();
}
