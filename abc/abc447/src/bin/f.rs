use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn dijkstra_all(s: usize, graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    let mut queue = VecDeque::new();
    result.insert(s, 0usize);
    queue.push_back((0, s));

    while let Some((step, i)) = queue.pop_front() {
        for &j in graph.get(&i).unwrap() {
            if !result.contains_key(&j) {
                result.insert(j, step + 1);
                queue.push_back((step + 1, j));
            }
        }
    }
    result
}
fn f(ab: &[(usize, usize)]) -> usize {
    let n = ab.len() + 1;

    let graph = build_ungraph(n, ab);

    // 連結数3以上のものが1つもなければ答え確定
    let mut result = 1;
    let count = graph.iter().filter(|v| v.len() >= 3).count();
    if count <= 0 {
        return result; // 全部直列 または 枝分かれ 1。 n >= 3 なので長さ 1 にはなる
    }

    if graph
        .iter()
        .filter(|&v| v.len() >= 3 && v.iter().any(|&w| graph[w].len() >= 3))
        .count()
        > 0
    {
        result = 2;
    }

    let mut visited = vec![false; n];
    for seed in 0..n {
        if visited[seed] {
            continue;
        }
        if graph[seed].len() < 4 {
            continue;
        }
        visited[seed] = true;
        let mut graph0 = HashMap::new();
        let mut stack = vec![seed];
        while let Some(u) = stack.pop() {
            visited[u] = true;
            for &v in &graph[u] {
                if visited[v] {
                    continue;
                }
                if graph[v].len() >= 4 {
                    // 伸ばせるので stack にも追加する
                    graph0.entry(u).or_insert(vec![]).push(v);
                    graph0.entry(v).or_insert(vec![]).push(u);
                    stack.push(v);
                } else if graph[v].len() == 3 {
                    // これ以上伸ばせない
                    graph0.entry(u).or_insert(vec![]).push(v);
                    graph0.entry(v).or_insert(vec![]).push(u);
                }
            }
        }

        if graph0.len() == 0 {
            continue; // ムカデの長さは増えない
        }

        // 木の直径を求める
        let dist0 = dijkstra_all(seed, &graph0);
        let mut step0 = 0;
        let mut i0 = seed; // seed から一番遠い点
        for (&i, &step) in &dist0 {
            if step >= step0 {
                i0 = i;
                step0 = step;
            }
        }

        let dist1 = dijkstra_all(i0, &graph0);
        let mut step1 = 0;
        for &step in dist1.values() {
            if step >= step1 {
                step1 = step;
            }
        }

        result = result.max(step1 + 1);
    }

    result
}

fn main() {
    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            n: usize,
            ab: [(Usize1, Usize1); n - 1],
        }
        let result = f(&ab);
        println!("{result}");
    }
}
