use proconio::{input, marker::Usize1};

fn build_digraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
    }
    graph
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let graph = build_digraph(n, &ab);
    let mut result = 0usize;
    for i in 0..n {
        let mut visited = vec![false; n];
        let mut stack = vec![i];
        while let Some(i) = stack.pop() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            for &i in &graph[i] {
                if !visited[i] {
                    stack.push(i);
                }
            }
        }
        result += visited.iter().filter(|&&x| x).count();
    }

    println!("{}", result);
}
