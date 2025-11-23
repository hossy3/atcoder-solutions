use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        d: usize,
    }
    let mut v = vec![a];
    while v[v.len() - 1] != b {
        let x = v[v.len() - 1] + d;
        v.push(x);
    }
    let result = v.iter().join(" ");
    println!("{result}");
}
