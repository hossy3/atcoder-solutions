use num_integer::Integer;
use proconio::input;

// 3角形の面積の2倍
fn area2((x0, y0): &(i64, i64), (x1, y1): &(i64, i64), (x2, y2): &(i64, i64)) -> i64 {
    let (dx0, dx1) = (x1 - x0, x2 - x1);
    let (dy0, dy1) = (y1 - y0, y2 - y1);
    dx0 * dy1 - dx1 * dy0
}

fn convex_hull_upper(xy: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut v = vec![xy[0]];
    for &xy in &xy[1..] {
        if v[v.len() - 1].0 == xy.0 {
            v.pop();
        }
        while v.len() >= 2 && area2(&v[v.len() - 2], &v[v.len() - 1], &xy) >= 0 {
            v.pop();
        }
        v.push(xy);
    }
    v
}

fn convex_hull_lower(xy: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut v = vec![xy[0]];
    for &xy in &xy[1..] {
        if v[v.len() - 1].0 == xy.0 {
            continue;
        }
        while v.len() >= 2 && area2(&v[v.len() - 2], &v[v.len() - 1], &xy) <= 0 {
            v.pop();
        }
        v.push(xy);
    }
    v
}

// ac_library::floor_sum() の $0 \leq a, b \leq m$ 制約を除いたもの
fn floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    let (a_div, a_mod) = a.div_mod_floor(&m);
    let (b_div, b_mod) = b.div_mod_floor(&m);

    a_div * n * (n - 1) / 2 + b_div * n + ac_library::floor_sum(n, m, a_mod, b_mod)
}

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    }

    xy.sort();
    let upper = convex_hull_upper(&xy);
    let lower = convex_hull_lower(&xy);

    let mut ngrid = 0i64;
    for v in upper.windows(2) {
        let ((x0, y0), (x1, y1)) = (v[0], v[1]);
        let m = x1 - x0;
        ngrid += floor_sum(m, m, y1 - y0, m * y0);
    }
    for v in lower.windows(2) {
        let ((x0, y0), (x1, y1)) = (v[0], v[1]);
        let m = x1 - x0;
        ngrid -= floor_sum(m, m, y1 - y0, m * y0 - 1);
    }
    ngrid += upper.last().unwrap().1 - lower.last().unwrap().1 + 1;

    let result = ngrid - n as i64;
    println!("{result}");
}
