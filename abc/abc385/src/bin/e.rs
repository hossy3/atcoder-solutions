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
        uv: [(Usize1, Usize1); n - 1],
    }

    let graph = build_ungraph(n, &uv);

    let mut use_max = 0usize;
    for i in 0..n {
        let mut v = vec![]; // 隣接するノードの外側エッジ数
        for &j in &graph[i] {
            v.push(graph[j].len() - 1);
        }
        v.sort();
        v.reverse();
        for (j, &x) in v.iter().enumerate() {
            if x == 0 {
                break;
            }
            use_max = use_max.max(1 + (j + 1) * (x + 1));
        }
    }

    let result = n - use_max;
    println!("{result}");
}
