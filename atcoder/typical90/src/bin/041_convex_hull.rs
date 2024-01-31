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

fn ngrid_on_edge((x0, y0): &(i64, i64), (x1, y1): &(i64, i64)) -> i64 {
    let dx = x1 - x0;
    let dy = y1 - y0;
    dx.gcd(&dy) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convex_hull() {
        let expected = vec![(1, 4), (6, 1), (5, 8)];
        assert_eq!(convex_hull(vec![(1, 4), (6, 1), (5, 8)]), expected);
        assert_eq!(convex_hull(vec![(1, 4), (5, 8), (6, 1)]), expected);
    }
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let v = convex_hull(xy);
    let convex_area2: i64 = v[1..]
        .windows(2)
        .map(|v0| area2(&v[0], &v0[0], &v0[1]))
        .sum();
    let ngrid_on_edges: i64 = (0..v.len())
        .map(|i| ngrid_on_edge(&v[i], &v[(i + 1) % v.len()]))
        .sum();
    let ngrid_on_loop = ngrid_on_edges + v.len() as i64;
    let result = (convex_area2 + ngrid_on_loop) / 2 - n as i64 + 1;
    println!("{result}");
}
