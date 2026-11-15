use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

// 子供の数を更新して返す
fn f0(graph: &[Vec<usize>], cur: usize, parent: usize, counts: &mut [usize]) {
    let mut count = 1usize;
    for &v in &graph[cur] {
        if v != parent {
            f0(graph, v, cur, counts);
            count += counts[v];
        }
    }
    counts[cur] = count;
}

// 累積和を子供に配る
fn f1(graph: &[Vec<usize>], cur: usize, parent: usize, results: &mut [isize]) {
    for &v in &graph[cur] {
        if v != parent {
            results[v] += results[cur];
            f1(graph, v, cur, results);
        }
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let graph = build_ungraph(n, &ab);

    let mut counts = vec![0usize; n];
    f0(&graph, 0, 0, &mut counts);

    let mut results = vec![0isize; n]; // いもす法適用前

    for (i, &(a, b)) in ab.iter().enumerate() {
        let i = (i as isize) + 1;
        if counts[a] < counts[b] {
            // a が下
            if counts[a] > n / 2 {
                results[a] += i;
            } else {
                results[0] += i;
                results[a] -= i;
            }
        } else {
            // b が下
            if counts[b] > n / 2 {
                results[b] += i;
            } else {
                results[0] += i;
                results[b] -= i;
            }
        }
    }

    f1(&graph, 0, 0, &mut results);

    println!("{}", results.iter().join(" "));
}
