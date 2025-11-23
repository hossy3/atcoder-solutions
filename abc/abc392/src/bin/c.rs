use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [Usize1; n],
    }

    let mut q_inv = vec![0; n]; // ゼッケンから人を辿る
    for (i, &x) in q.iter().enumerate() {
        q_inv[x] = i;
    }

    let mut results = vec![];
    for i in 0..n {
        results.push(q[p[q_inv[i]]] + 1);
    }
    let result = results.iter().join(" ");
    println!("{result}");
}
