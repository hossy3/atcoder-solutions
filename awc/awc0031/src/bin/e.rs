use ac_library::MfGraph;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        b: [usize; n],
        w: [usize; k],
        uvc: [(Usize1, Usize1, usize); m],
        q: usize,
        s: [Usize1; q],
    }

    let x = n + k;
    let y = n + k + 1;

    let b_sum = b.iter().sum::<usize>();
    let mut w_enable = vec![true; k];

    for s in s {
        w_enable[s] = false;

        let mut graph = MfGraph::new(n + k + 2);
        for (i, &b) in b.iter().enumerate() {
            graph.add_edge(x, i, b);
        }
        for (i, &w) in w.iter().enumerate() {
            if w_enable[i] {
                graph.add_edge(n + i, y, w);
            };
        }
        for &(u, v, c) in &uvc {
            graph.add_edge(u, v, c);
            graph.add_edge(v, u, c);
        }

        let flow = graph.flow(x, y);
        let yes = flow == b_sum;

        println!("{}", if yes { "Yes" } else { "No" });
    }
}
