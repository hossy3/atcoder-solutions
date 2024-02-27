use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra_all;
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

fn main() {
    input! {
        n: usize,
        a: [char; n],
        ab: [(Usize1, Usize1); n - 1],
    }

    let graph = build_ungraph(n, &ab);
    let reachables = dijkstra_all(&0, |&i| graph[i].iter().map(|&j| (j, 1usize)));

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

    let mut stack: Vec<_> = reachables
        .iter()
        .sorted_by_key(|(&_, &(_, cost))| cost)
        .collect();

    while let Some((&i, &(parent, _))) = stack.pop() {
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
