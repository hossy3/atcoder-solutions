use ac_library::SccGraph;
use proconio::{input, marker::Usize1};

type Mint = ac_library::ModInt998244353;

fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
    }
    graph
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            uv: [(Usize1, Usize1); m],
        }

        let mut graph = SccGraph::new(n);
        for &(u, v) in &uv {
            graph.add_edge(u, v);
        }
        let scc = graph.scc();

        let graph = build_digraph(n, &uv);

        let mut dp = vec![Mint::new(0); n];
        dp[0] += 1;
        for scc in scc {
            for u in scc {
                for &v in &graph[u] {
                    let x = dp[u];
                    dp[v] += x;
                }
            }
        }

        let result = dp[n - 1];
        println!("{result}");
    }
}
