use std::collections::BTreeMap;

use ac_library::MfGraph;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, t): (usize, i64),
        axy: [(i64, i64); n],
        bxy: [(i64, i64); n],
    }

    let dirs = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut bi_map = BTreeMap::new();
    for (i, &(bx, by)) in bxy.iter().enumerate() {
        bi_map.insert((bx, by), i);
    }

    let mut ijk_map = BTreeMap::new();

    let mut graph = MfGraph::<usize>::new(n * 2 + 2);
    let sn = n * 2;
    let tn = n * 2 + 1;

    for (i, &(ax, ay)) in axy.iter().enumerate() {
        graph.add_edge(sn, i, 1);
        for (k, &(dx, dy)) in dirs.iter().enumerate() {
            let (x, y) = (ax + dx * t, ay + dy * t);
            if let Some(&j) = bi_map.get(&(x, y)) {
                graph.add_edge(i, n + j, 1);
                ijk_map.insert((i, n + j), k);
            }
        }
    }
    for (j, _) in bxy.iter().enumerate() {
        graph.add_edge(n + j, tn, 1);
    }

    let flow = graph.flow(sn, tn);
    let yes = flow == n;
    println!("{}", if yes { "Yes" } else { "No" });

    if yes {
        let mut results = vec![0; n];
        for edge in &graph.edges() {
            if edge.flow == 1 {
                if let Some(k) = ijk_map.get(&(edge.from, edge.to)) {
                    results[edge.from] = k + 1;
                }
            }
        }
        let result = results.iter().join(" ");
        println!("{result}");
    }
}
