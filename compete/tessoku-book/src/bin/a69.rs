use proconio::{input, marker::Chars};

// Bipartite matching
// だいたいテキストの写経

fn dfs(
    pos: usize,
    goal: usize,
    f: usize,
    g: &mut [Vec<(usize, usize, usize)>],
    visited: &mut [bool],
) -> usize {
    if pos == goal {
        return f;
    }
    visited[pos] = true;

    for i in 0..g[pos].len() {
        let (to, cap, j) = g[pos][i];
        if cap == 0 || visited[to] {
            continue;
        }
        let flow = dfs(to, goal, f.min(cap), g, visited);
        if flow >= 1 {
            g[pos][i].1 -= flow;
            g[to][j].1 += flow;
            return flow;
        }
    }
    0
}

fn f(a: usize, b: usize, g: &mut [Vec<(usize, usize, usize)>]) {
    let na = g[a].len();
    let nb = g[b].len();
    g[a].push((b, 1, nb));
    g[b].push((a, 0, na));
}

fn main() {
    input! {
        n: usize,
        c: [Chars; n],
    }

    let mut g = vec![vec![]; 2 * n + 2]; // (node, capacity, rev_edge)
    let last = 1 + 2 * n;

    for i in 0..n {
        f(0, 1 + i, &mut g);
    }
    for i in 0..n {
        for j in 0..n {
            if c[i][j] == '#' {
                f(1 + i, 1 + n + j, &mut g);
            }
        }
    }
    for i in 0..n {
        f(1 + n + i, last, &mut g);
    }

    let mut total = 0;
    loop {
        let mut visited = vec![false; last + 1];
        let f = dfs(0, last, n + 1, &mut g, &mut visited);
        if f == 0 {
            println!("{}", total);
            return;
        }
        total += f;
    }
}
