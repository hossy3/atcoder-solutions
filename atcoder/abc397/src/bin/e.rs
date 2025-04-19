use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

// Tree DP
fn f(k: usize, graph: &[Vec<usize>]) -> bool {
    let mut stack = vec![(0, 0, true)];
    let mut counts = vec![1usize; graph.len()];
    while let Some((node, parent, traversing)) = stack.pop() {
        if traversing {
            stack.push((node, parent, false));
            for &child in &graph[node] {
                if child != parent {
                    stack.push((child, node, true));
                }
            }
            continue;
        } 

        let mut connection = 0usize;
        for &child in &graph[node] {
            if child != parent {
                counts[node] += counts[child];
                if counts[child] > 0 {
                    connection += 1;
                }
            }
        }

        let count = counts[node];
        if connection >= 3 || (connection == 2 && count < k) || count > k {
            return false;
        }
        counts[node] = count % k;
    } 

    true
}

fn main() {
    input! {
        n: usize,
        k: usize,
        uv: [(Usize1, Usize1); n * k - 1],
    }

    let graph = build_ungraph(n * k, &uv);
    let yes = f(k, &graph);
    println!("{}", if yes { "Yes" } else { "No" });
}
