use itertools::Itertools;
use proconio::input;

fn f((x0, y0): (i64, i64), (x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> bool {
    let (dx1, dy1) = (x1 - x0, y1 - y0);
    let (dx2, dy2) = (x2 - x0, y2 - y0);
    dx1 * dy2 == dx2 * dy1
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let yes = (0..n)
        .permutations(3)
        .any(|v| f(xy[v[0]], xy[v[1]], xy[v[2]]));
    println!("{}", if yes { "Yes" } else { "No" });
}
