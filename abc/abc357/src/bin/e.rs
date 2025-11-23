use std::collections::HashSet;

use ac_library::SccGraph;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut graph = SccGraph::new(n);
    for (i, &j) in a.iter().enumerate() {
        graph.add_edge(i, j);
    }
    let scc = graph.scc();

    let mut counts = vec![0usize; n];
    for v in scc.iter().rev() {
        let mut set = HashSet::new();
        for x in v {
            set.insert(x);
        }

        let mut count = 0;
        for &x in v {
            let x = a[x];
            if !set.contains(&x) {
                count = counts[x];
                break;
            }
        }
        count += v.len();

        for &x in v {
            counts[x] = count;
        }
    }

    let result: usize = counts.iter().sum();
    println!("{result}");
}
