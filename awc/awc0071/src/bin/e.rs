use ac_library::{Dsu, MfGraph};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    // 2部グラフを組み立てる
    let mut dsu = Dsu::new(n * 2);
    for &(u, v) in &uv {
        dsu.merge(u, v + n);
        dsu.merge(v, u + n);
    }
    for &(u, v) in &uv {
        if !dsu.same(u, 0) && !dsu.same(v, 0) {
            dsu.merge(u, 0); // 連結でない場合
        }
    }

    let mut graph = MfGraph::new(n + 2);
    let s = n;
    let t = n + 1;
    for u in 0..n {
        if dsu.same(u, 0) {
            graph.add_edge(s, u, 1);
        } else {
            graph.add_edge(u, t, 1);
        }
    }
    for &(u, v) in &uv {
        let (u, v) = if dsu.same(u, 0) { (u, v) } else { (v, u) };
        graph.add_edge(u, v, 1);
    }

    let result = graph.flow(s, t);
    println!("{result}");
}
