use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn shortest_all_ungraph(s: usize, graph: &[Vec<usize>]) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    v[s] = Some(0usize);
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        for &j in &graph[i] {
            if v[j].is_none() {
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
    }

    let mut graph = vec![vec![]; n + m];
    for i in 0..m {
        let j = n + i;

        input! {
            k: usize,
            r: [Usize1; k],
        }
        for x in r {
            graph[j].push(x);
            graph[x].push(j);
        }
    }

    let v = shortest_all_ungraph(0, &graph);
    for &x in v[..n].iter() {
        if let Some(x) = x {
            let result = x / 2;
            println!("{result}");
        } else {
            println!("-1");
        }
    }
}
