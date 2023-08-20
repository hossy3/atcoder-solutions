use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
        c: [Usize1; n],
    }
    let mut v = vec![(0usize, 0usize); n];
    for &a in &a {
        v[a].0 += 1;
    }
    for &c in &c {
        v[b[c]].1 += 1;
    }
    let result: usize = v.iter().map(|(a, b)| a * b).sum();
    println!("{}", result);
}
