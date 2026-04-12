use ac_library::Dsu;
use itertools::Itertools;
use proconio::input;

/// 非負の重み付き木グラフの s から e までの最短距離を DFS で解く
fn shortest_graph(graph: &[Vec<(usize, usize)>], s: usize, e: usize) -> Option<usize> {
    let mut stack = vec![(s, s, 0)]; // cur, parnt, cost
    while let Some((cur, parent, cost)) = stack.pop() {
        if cur == e {
            return Some(cost);
        }
        for &(j, w) in &graph[cur] {
            if j != parent {
                stack.push((j, cur, cost + w));
            }
        }
    }
    None
}

fn f(a: &[Vec<usize>]) -> bool {
    let n = a.len() + 1;

    let mut v = vec![];
    for i in 0..(n - 1) {
        for j in 0..(n - i - 1) {
            v.push((a[i][j], i, i + j + 1));
        }
    }

    let mut graph = vec![vec![]; n]; // 無向グラフ
    let mut dsu = Dsu::new(n);

    for &(x, i, j) in v.iter().sorted() {
        if dsu.same(i, j) {
            let dist = shortest_graph(&graph, i, j);
            if dist != Some(x) {
                return false;
            }
        } else {
            graph[i].push((j, x));
            graph[j].push((i, x));
            dsu.merge(i, j);
        }
    }

    true
}

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    for i in 1..n {
        input! {
            a0: [usize; n - i],
        }
        a.push(a0);
    }

    let yes = f(&a);
    println!("{}", if yes { "Yes" } else { "No" });
}
