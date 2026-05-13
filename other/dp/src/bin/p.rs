use proconio::{input, marker::Usize1};

type Mint = ac_library::ModInt1000000007;

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

/// 木DP (Tree DP)
fn tree_dp(graph: &Vec<Vec<usize>>, cur: usize, parent: usize, state: &mut Vec<(Mint, Mint)>) {
    for &child in &graph[cur] {
        if child != parent {
            tree_dp(graph, child, cur, state);
            state[cur].0 = state[cur].0 * (state[child].0 + state[child].1); // 白: 子はなんでも良い
            state[cur].1 = state[cur].1 * state[child].0; // 黒: 子は白のみ
        }
    }
}

fn main() {
    input! {
        n: usize,
        xy: [(Usize1, Usize1); n - 1],
    }

    let graph = build_ungraph(n, &xy);
    let mut state = vec![(Mint::new(1), Mint::new(1)); n];
    tree_dp(&graph, 0, usize::MAX, &mut state);
    let result = state[0].0 + state[0].1;
    println!("{result}");
}
