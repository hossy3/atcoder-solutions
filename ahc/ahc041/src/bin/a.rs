use itertools::Itertools;
use proconio::input;
use rand::Rng;

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

// rand_factor: 100 なら 100% ゆらぎあり
fn f(h: usize, a: &[usize], graph: &[Vec<usize>], rand_factor: usize) -> (Vec<usize>, usize) {
    let n = graph.len();

    let mut rng = rand::thread_rng();

    let mut parents = vec![usize::MAX; n];
    let mut used = 0;
    let mut score = 1;
    while used < n {
        // 一番スコアが小さなノードを始点とする
        let mut start = usize::MAX;
        let mut start_x = usize::MAX;
        for i in 0..n {
            let x = if a[i] == usize::MAX {
                usize::MAX
            } else {
                100 + rng.gen_range(0..=rand_factor)
            };
            if parents[i] == usize::MAX && (start == usize::MAX || x < start_x) {
                start = i;
                start_x = x;
            }
        }

        let mut stack = vec![(start, start, 0)]; // 自分, 親, 深さ

        while let Some((node, parent, depth)) = stack.pop() {
            if parents[node] != usize::MAX {
                continue; // すでに使用済み
            }
            parents[node] = parent;
            score += a[node] * (depth + 1);
            used += 1;
            if depth == h {
                continue;
            }

            let mut v = vec![];
            for &node in &graph[node] {
                if parents[node] != usize::MAX {
                    continue;
                }
                v.push((a[node], node));
            }
            v.sort();
            v.reverse(); // スコアの高い順に並べる (高いスコアほど後に使うようにする)

            for (_, child) in v {
                stack.push((child, node, depth + 1));
            }
        }
    }

    (parents, score)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        h: usize,
        a: [usize; n],
        uv: [(usize, usize); m],
        _xy: [(usize, usize); n],
    }

    let graph = build_ungraph(n, &uv);
    let (mut parents, mut score) = f(h, &a, &graph, 0);

    for i in 1..=200 {
        for _ in 0..(200 / i) {
            let (parents0, score0) = f(h, &a, &graph, i);
            if score < score0 {
                parents = parents0;
                score = score0;
            }
        }
    }

    let mut results = vec![];
    for i in 0..n {
        let x = if parents[i] == i {
            -1
        } else {
            parents[i] as isize
        };
        results.push(x);
    }
    let result = results.iter().join(" ");
    println!("{result}");
    eprintln!("{score}");
}
