use ac_library::MfGraph;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        w: usize,
        a: [usize; n],
    }
    let mut m = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            k: usize,
            c: [Usize1; k],
        }
        m.push(c);
    }

    let mut graph = MfGraph::<usize>::new(n + 2);
    let s = n;
    let t = n + 1;

    for (i, c) in m.iter().enumerate() {
        for &c in c {
            graph.add_edge(c, i, usize::MAX); // 鍵がかかっている c には先に入れない
        }
    }
    for (i, &a) in a.iter().enumerate() {
        graph.add_edge(s, i, a); // 家に入らないときに a 円得られなくなる
        graph.add_edge(i, t, w); // 家に入るときに w 円消費する
    }
    let flow = graph.flow(s, t);
    let result = a.iter().sum::<usize>() - flow;
    println!("{result}");
}
