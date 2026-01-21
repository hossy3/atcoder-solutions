use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn shortest_all_ungraph(s: usize, graph: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    v[s] = Some(0usize);
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        for &j in &graph[i] {
            if v[j] == None {
                v[j] = Some(step + 1);
                queue.push_back((step + 1, j));
            }
        }
    }
    v
}

fn compress_coordinates(a: &[usize]) -> Vec<usize> {
    let mut sorted = a.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut map = std::collections::HashMap::new();
    for (i, &x) in sorted.iter().enumerate() {
        map.insert(x, i);
    }
    a.iter().map(|&x| map[&x]).collect()
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        uv: [(Usize1, Usize1); n - 1],
    }
    let graph = build_ungraph(n, &uv);
    let shortest: Vec<_> = shortest_all_ungraph(0, &graph)
        .iter()
        .map(|&x| x.unwrap())
        .collect();
    let a = compress_coordinates(&a);

    let mut results = vec![usize::MAX; n];
    let mut stack = vec![0];
    let mut undo = vec![];
    let mut lis = vec![];

    while let Some(i) = stack.pop() {
        while undo.len() > shortest[i] {
            let (j, x) = undo.pop().unwrap();
            if x == usize::MAX {
                lis.pop();
            } else {
                lis[j] = x;
            }
        }

        let x = a[i];
        let j = lis.partition_point(|&y| y < x);
        if j == lis.len() {
            undo.push((j, usize::MAX));
            lis.push(x);
        } else if lis[j] > x {
            undo.push((j, lis[j]));
            lis[j] = x;
        }
        results[i] = lis.len();

        for &j in &graph[i] {
            if shortest[i] < shortest[j] {
                stack.push(j);
            }
        }
    }

    for result in results {
        println!("{result}");
    }
}
