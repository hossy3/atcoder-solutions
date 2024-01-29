use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

// nums: 各ノードの、自分自身と子孫の合計数
fn dfs(i: usize, parent: usize, graph: &[Vec<usize>], nums: &mut [usize]) -> usize {
    let mut count = 1;
    for &j in &graph[i] {
        if j != parent {
            count += dfs(j, i, graph, nums);
        }
    }
    nums[i] = count;
    count
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1]
    }

    let graph = build_ungraph(n, &ab);
    let mut nums = vec![0; n];
    dfs(0, 0, &graph, &mut nums);

    let mut count = 0;
    for &(a, b) in &ab {
        let x = nums[a].min(nums[b]);
        count += x * (n - x);
    }
    println!("{count}");
}
