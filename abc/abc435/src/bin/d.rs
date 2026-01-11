use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[v].push(u);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
        q: usize,
    }

    let graph = build_digraph(n, &xy);
    let mut nodes = vec![false; n]; // 白 false, 黒 true

    for _ in 0..q {
        input! {
            t: u8,
            v: Usize1,
        }

        if t == 1 {
            if nodes[v] {
                continue;
            }
            let mut stack = vec![v];
            nodes[v] = true;
            while let Some(v) = stack.pop() {
                for &v in &graph[v] {
                    if !nodes[v] {
                        stack.push(v);
                        nodes[v] = true;
                    }
                }
            }
        } else if t == 2 {
            let yes = nodes[v];
            println!("{}", if yes { "Yes" } else { "No" });
        } else {
            unreachable!();
        }
    }
}
