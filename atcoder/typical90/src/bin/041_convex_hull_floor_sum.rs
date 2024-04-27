use itertools::Itertools;
use num_integer::Integer;
use proconio::input;

// 3角形の面積の2倍
fn area2((x0, y0): &(i64, i64), (x1, y1): &(i64, i64), (x2, y2): &(i64, i64)) -> i64 {
    let (dx0, dx1) = (x1 - x0, x2 - x1);
    let (dy0, dy1) = (y1 - y0, y2 - y1);
    dx0 * dy1 - dx1 * dy0
}

// 凸包を x 最小値から反時計回りに返す
fn convex_hull(mut xy: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    assert!(xy.len() >= 3);
    xy.sort();

    let mut lower = vec![xy[0], xy[1]];
    for &xy in &xy[2..] {
        while lower.len() >= 2 && area2(&lower[lower.len() - 2], &lower[lower.len() - 1], &xy) <= 0
        {
            lower.pop();
        }
        lower.push(xy);
    }

    let mut upper = vec![xy[0], xy[1]];
    for &xy in &xy[2..] {
        while upper.len() >= 2 && area2(&upper[upper.len() - 2], &upper[upper.len() - 1], &xy) >= 0
        {
            upper.pop();
        }
        upper.push(xy);
    }

    for &x in upper[1..(upper.len() - 1)].iter().rev() {
        lower.push(x);
    }
    lower
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
        xy: [(i64, i64); n],
    }

    let v = convex_hull(xy);

    let lower_min = 0;
    let upper_min = v.iter().position_max().unwrap();
    let lower_max = if v[upper_min].0 == v[upper_min - 1].0 {
        upper_min - 1
    } else {
        upper_min
    };
    let upper_max = if v[0].0 == v[v.len() - 1].0 {
        v.len() - 1
    } else {
        v.len()
    };

    let mut ngrid = 0i64;
    for i in (upper_min..upper_max).rev() {
        let (x0, y0) = &v[(i + 1) % v.len()];
        let (x1, y1) = &v[i];
        ngrid += (x1 - x0) * y0 + floor_sum(x1 - x0, x1 - x0, y1 - y0, 0);
    }
    for i in lower_min..lower_max {
        let (x0, y0) = &v[i];
        let (x1, y1) = &v[(i + 1) % v.len()];
        ngrid -= (x1 - x0) * y0 + floor_sum(x1 - x0, x1 - x0, y1 - y0, -1);
    }
    ngrid += v[upper_min].1 - v[lower_max].1 + 1;

    let result = ngrid - n as i64;
    println!("{result}");
}
