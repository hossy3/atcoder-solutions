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
        k: usize,
        t: usize,
        c: [Usize1; k],
        uv: [(Usize1, Usize1); m],
    }

    let graph = build_ungraph(n, &uv);

    let mut known = vec![0; n]; // 友人のうち知っている数
    for &c in &c {
        known[c] = t; // 便宜的に判定条件と同じにする
    }

    // 問題文に「同時更新」と書いているけれど、同時でなくても伝搬結果は同じはず。実装しやすい後者で進める
    let mut stack = c.clone();
    while let Some(u) = stack.pop() {
        for &v in &graph[u] {
            if known[v] >= t {
                continue;
            }
            known[v] += 1;
            if known[v] == t {
                stack.push(v);
            }
        }
    }

    let result = known.iter().filter(|&&x| x >= t).count();
    println!("{result}");
}
