use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap},
};

use proconio::{input, marker::Usize1};

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

fn solve(graph: &[Vec<usize>], a: &[usize]) -> usize {
    let n = a.len();
    let mut scores = vec![0usize; a.len()];
    scores[0] = 1;

    let mut set = BTreeSet::new();
    set.insert((Reverse(a[0]), 1usize, 0usize)); // (x, score, index)

    let mut map = HashMap::new();
    map.insert(0usize, 1usize);

    while let Some((Reverse(x), score, i)) = set.pop_last() {
        if i == n - 1 {
            return score;
        }
        if scores[i] > score {
            continue;
        }

        for &j in &graph[i] {
            let x0 = a[j];
            if x0 < x {
                continue;
            }
            let score0 = if x0 == x { score } else { score + 1 };
            if score0 < score || (x0 == x && scores[j] == score) {
                continue;
            }
            if let Some(&score1) = map.get(&j) {
                if score1 >= score0 {
                    continue;
                }
                set.remove(&(Reverse(x0), score1, j));
            }
            scores[j] = score0;
            set.insert((Reverse(x0), score0, j));
            map.insert(j, score0);
        }
    }

    scores[n - 1]
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        uv: [(Usize1, Usize1); m],
    }

    let graph = build_ungraph(n, &uv);
    let result = solve(&graph, &a);
    println!("{result}");
}
