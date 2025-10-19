use proconio::{
    input,
    marker::{Chars, Usize1},
};

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
            k: usize,
            s: Chars,
            uv: [(Usize1, Usize1); m],
        }

        let graph = build_digraph(n, &uv);

        // true: Alice の勝ち, false: Bob の勝ち
        let mut dp = vec![vec![true; n]; k * 2 + 1];
        for (i, &c) in s.iter().enumerate() {
            dp[0][i] = c == 'A';
        }

        for i in 0..k {
            // Bob の行動
            for j in 0..n {
                dp[i * 2 + 1][j] = graph[j].iter().all(|&j| dp[i * 2][j]);
            }

            // Alice の行動
            for j in 0..n {
                dp[i * 2 + 2][j] = graph[j].iter().any(|&j| dp[i * 2 + 1][j]);
            }
        }
        let yes = dp[k * 2][0];
        println!("{}", if yes { "Alice" } else { "Bob" });
    }
}
