use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut uf = Dsu::new(n * 2);
    for &(a, b) in ab.iter() {
        uf.merge(a, n + b);
        uf.merge(n + a, b);
    }

    let mut counts = (0usize, 0usize);
    for i in 0..n {
        if uf.same(i, 0) {
            counts.0 += 1;
        } else {
            counts.1 += 1;
        }
    }
    let x = if counts.0 >= counts.1 { 0 } else { n };

    let mut results = vec![];
    for i in 0..n {
        if uf.same(i, x) {
            results.push(i);
        }
    }

    let result = results[..(n / 2)].iter().map(|x| x + 1).join(" ");
    println!("{result}");
}
