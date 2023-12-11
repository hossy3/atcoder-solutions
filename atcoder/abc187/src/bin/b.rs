use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let result = (0..n)
        .tuple_combinations()
        .filter(|&(i, j)| (xy[j].0 - xy[i].0).abs() >= (xy[j].1 - xy[i].1).abs())
        .count();
    println!("{result}");
}
