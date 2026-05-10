use proconio::{input, marker::Usize1};

/// 重みなしグラフの s から e までの最短距離を 01-bfs で解く
fn shortest_graph(graph: &[Vec<usize>], s: usize, e: usize) -> Option<usize> {
    use std::collections::{HashSet, VecDeque};

    let mut set = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        if i == e {
            return Some(step);
        }
        for &j in &graph[i] {
            if set.insert(j) {
                queue.push_back((step + 1, j));
            }
        }
    }
    None
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
    }

    let mut graph = vec![vec![]; m + n]; // ..n: バス停, n..: バス路線
    for i in 0..m {
        input! {
            k: usize,
            a: [Usize1; k],
        }

        let j = i + n; // バス路線の番号
        for a in a {
            graph[a].push(j);
            graph[j].push(a);
        }
    }

    if let Some(len) = shortest_graph(&graph, s, t) {
        let result = len / 2; // バス路線は数に含めない
        println!("{result}");
    } else {
        println!("-1");
    }
}
