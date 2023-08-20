use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut vx = xy.iter().map(|(x, _)| x).collect_vec();
    let mut vy = xy.iter().map(|(_, y)| y).collect_vec();
    vx.sort();
    vy.sort();
    let mx = vx[n / 2];
    let my = vy[n / 2];
    let score = xy
        .iter()
        .fold(0, |acc, (x, y)| acc + (x - mx).abs() + (y - my).abs());
    println!("{}", score);
}
