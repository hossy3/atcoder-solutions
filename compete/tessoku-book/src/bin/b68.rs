use proconio::input;

// Minimum Flow (Ford-Fulkerson)

fn dfs(
    pos: usize,
    goal: usize,
    flow: i64,
    graph: &mut [Vec<(usize, i64, usize)>],
    visited: &mut [bool],
) -> i64 {
    if pos == goal {
        return flow;
    }
    visited[pos] = true;

    for i in 0..graph[pos].len() {
        let (to, cap, j) = graph[pos][i];
        if cap == 0 || visited[to] {
            continue;
        }
        let flow = dfs(to, goal, flow.min(cap), graph, visited);
        if flow >= 1 {
            graph[pos][i].1 -= flow;
            graph[to][j].1 += flow;
            return flow;
        }
    }
    0
}

// a -- cap --> b
fn f(a: usize, b: usize, cap: i64, graph: &mut [Vec<(usize, i64, usize)>]) {
    let na = graph[a].len();
    let nb = graph[b].len();
    graph[a].push((b, cap, nb));
    graph[b].push((a, 0, na));
}

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [i64; n],
        ab: [(usize, usize); m],
    }

    let n = n + 2;
    let mut graph = vec![vec![]; n]; // (node, capacity, rev_edge)

    let mut offset = 0i64;
    for (i, &p) in p.iter().enumerate() {
        let i = i + 1;
        let s = 0;
        let t = n - 1;
        if p > 0 {
            offset += p;
            f(s, i, p, &mut graph);
            f(i, t, 0, &mut graph);
        } else {
            f(s, i, 0, &mut graph);
            f(i, t, -p, &mut graph);
        }
    }

    const MAX: i64 = 1 << 20;
    for (a, b) in ab {
        f(a, b, MAX, &mut graph);
    }

    let mut total = 0;
    loop {
        let mut visited = vec![false; n];
        let f = dfs(0, n - 1, 10000, &mut graph, &mut visited);
        if f == 0 {
            let result = offset - total;
            println!("{}", result);
            return;
        }
        total += f;
    }
}
