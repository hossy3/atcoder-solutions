use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, abw: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(a, b, w) in abw {
        graph[a].push((b, w));
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abw: [(Usize1, Usize1, usize); m],
    }
    const N: usize = 1024; // 2 ** 10

    let graph = build_digraph(n, &abw);
    let mut state = vec![vec![false; N]; n];
    state[0][0] = true;
    let mut stack = vec![(0usize, 0usize)]; // index, w

    while let Some((i, w)) = stack.pop() {
        for &(j, w0) in &graph[i] {
            let w = w ^ w0;
            if !state[j][w] {
                state[j][w] = true;
                stack.push((j, w));
            }
        }
    }

    for (i, &x) in state[n - 1].iter().enumerate() {
        if x {
            println!("{i}");
            return;
        }
    }
    println!("-1");
}
