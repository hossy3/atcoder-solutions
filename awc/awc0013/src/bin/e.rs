use ac_library::MfGraph;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut graph = MfGraph::<usize>::new(n + m + 2);
    let s = n + m;
    let t = n + m + 1;
    for i in 0..n {
        graph.add_edge(s, i, 1);
    }
    for i in 0..m {
        graph.add_edge(n + i, t, 1);
    }
    for i in 0..n {
        input! {
            c: [Usize1],
        }
        for j in c {
            graph.add_edge(i, n + j, 1);
        }
    }

    let result = graph.flow(s, t);
    println!("{result}");
}
