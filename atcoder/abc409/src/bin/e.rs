use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uvw: &[(usize, usize, usize)]) -> Vec<HashMap<usize, usize>> {
    let mut graph = vec![HashMap::new(); n]; // node, edge, weight
    for &(u, v, w) in uvw {
        graph[u].insert(v, w);
        graph[v].insert(u, w);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        mut x: [i64; n],
        uvw: [(Usize1, Usize1, usize); n - 1],
    }

    let mut graph = build_ungraph(n, &uvw);
    let mut stack = vec![];
    for i in 0..n {
        if graph[i].len() == 1 {
            stack.push(i);
        }
    } 

    let mut result = 0;
    while let Some(i) = stack.pop() {
        let Some((&j, &w)) = graph[i].iter().next() else { continue; }; 
        x[j] += x[i];
        result += x[i].abs() as usize * w;
        graph[i].remove(&j);
        graph[j].remove(&i);
        if graph[j].len() == 1 {
            stack.push(j);
        }
    }
    println!("{result}");
}
