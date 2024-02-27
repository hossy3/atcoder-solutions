use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

type Mint = ac_library::ModInt1000000007;

#[derive(Clone)]
struct State {
    ab: Mint,
    a: Mint,
    b: Mint,
}

impl State {
    fn new() -> Self {
        Self {
            ab: Mint::new(0),
            a: Mint::new(0),
            b: Mint::new(0),
        }
    }
}

fn build_ungraph(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; n]; // node, edge
    for &(u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

// Returns Vec<(step, parent)>
fn dijkstra_all(s: usize, graph: &[Vec<usize>]) -> Vec<Option<(usize, usize)>> {
    let n = graph.len();
    let mut v = vec![None; n];
    let mut queue = VecDeque::new();
    queue.push_back((0usize, s));

    while let Some((step, i)) = queue.pop_front() {
        for &j in &graph[i] {
            if j == s || v[j].is_some() {
                continue;
            }
            v[j] = Some((step + 1, i));
            queue.push_back((step + 1, j));
        }
    }
    v
}

fn main() {
    input! {
        n: usize,
        a: [char; n],
        ab: [(Usize1, Usize1); n - 1],
    }

    let graph = build_ungraph(n, &ab);
    let tree = dijkstra_all(0, &graph);

    let mut v = vec![State::new(); n];
    for (i, &c) in a.iter().enumerate() {
        match c {
            'a' => {
                v[i].a += 1;
            }
            'b' => {
                v[i].b += 1;
            }
            _ => unreachable!(),
        }
    }

    let mut stack: Vec<_> = tree
        .iter()
        .enumerate()
        .sorted_by_key(|&(i, x)| (x, i))
        .filter_map(|(i, x)| x.and_then(|(_, parent)| Some((i, parent))))
        .collect();

    while let Some((i, parent)) = stack.pop() {
        let ncut = State {
            ab: v[parent].ab * v[i].ab,
            a: v[parent].a * v[i].ab,
            b: v[parent].b * v[i].ab,
        };
        let nconnect = State {
            ab: v[parent].ab * (v[i].ab + v[i].a + v[i].b)
                + v[parent].a * (v[i].ab + v[i].b)
                + v[parent].b * (v[i].ab + v[i].a),
            a: v[parent].a * v[i].a,
            b: v[parent].b * v[i].b,
        };
        v[parent] = State {
            ab: ncut.ab + nconnect.ab,
            a: ncut.a + nconnect.a,
            b: ncut.b + nconnect.b,
        };
    }

    let result = v[0].ab;
    println!("{result}");
}
