use ac_library::Dsu;
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

    // 頂点数 3 のグラフの隣接関係を調べる
    let mut dsu = Dsu::new(n);
    for (i, v) in graph.iter().enumerate() {
        if v.len() != 3 {
            continue;
        }
        for &j in v {
            if graph[j].len() != 3 {
                continue;
            }
            dsu.merge(i, j);
        }
    }

    // dsu から次数3の隣接したかたまりを組み立てる
    let mut v0 = vec![vec![]; n];
    for (i, v) in graph.iter().enumerate() {
        if v.len() != 3 {
            continue;
        }
        let k = dsu.leader(i);
        v0[k].push(i);
    }

    let mut result = 0usize;
    for v in v0 {
        let mut count = 0; // 次数2の隣接した数 (木なので重複しないはず)
        for x in v {
            for &i in &graph[x] {
                if graph[i].len() != 2 {
                    continue;
                }
                count += 1;
            }
        }
        if count > 0 {
            result += count * (count - 1) / 2;
        }
    }

    println!("{result}");
}
