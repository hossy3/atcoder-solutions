use ac_library::MfGraph;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [usize; m],
    }

    let mut c = vec![];
    for _ in 0..n {
        input! {
            k: usize,
            c0: [Usize1; k],
        }
        c.push(c0);
    }

    let s = 0;
    let t = 1 + m + n;
    let mut graph = MfGraph::new(t + 1);

    for (i, &b) in b.iter().enumerate() {
        graph.add_edge(i + 1, t, b); // 渡せるプレゼントを流量で表す
        // eprintln!("{}, {}, {}", i + 1, t, b);
    }
    for (i, c) in c.iter().enumerate() {
        let k = 1 + m + i;
        graph.add_edge(s, k, 1);
        for &c in c {
            let b = c + 1;
            graph.add_edge(k, b, 1);
        }
    }

    let result = graph.flow(s, t);
    println!("{result}");
}
