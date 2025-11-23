use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, abc: &[(usize, usize, usize)]) -> Vec<Vec<(usize, usize)>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(a, b, c) in abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }
    graph
}

fn longest_path(i: usize, graph: &[Vec<(usize, usize)>], visited: &mut [bool; 10]) -> usize {
    let mut result = 0;
    for &(j, c) in &graph[i] {
        if !visited[j] {
            visited[j] = true;
            result = result.max(c + longest_path(j, graph, visited));
            visited[j] = false;
        }
    }
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let graph = build_ungraph(n, &abc);
    let mut result = 0;
    for i in 0..n {
        let mut visited = [false; 10];
        visited[i] = true;
        result = result.max(longest_path(i, &graph, &mut visited));
    }
    println!("{result}");
}
