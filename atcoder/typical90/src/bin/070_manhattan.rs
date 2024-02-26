use itertools::Itertools;
use proconio::input;

fn f(v: &[i64]) -> i64 {
    let n = v.len();
    let m = v[n / 2];
    let sum = v.iter().map(|&x| (m - x).abs()).sum();
    sum
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let vx: Vec<_> = xy.iter().map(|&(x, _)| x).sorted().collect();
    let vy: Vec<_> = xy.iter().map(|&(_, y)| y).sorted().collect();
    let result = f(&vx) + f(&vy);
    println!("{result}");
}
