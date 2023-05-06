use itertools::Itertools;
use proconio::input;

fn f(xy: &[(i64, i64)]) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let n = xy.len();

    let pos_x_min = xy.iter().position_min().unwrap();
    let pos_x_max = xy.iter().position_max().unwrap();

    let mut v0 = vec![]; // lower
    let i_min = pos_x_min;
    let mut i_max = pos_x_max + if pos_x_max < pos_x_min { n } else { 0 };
    if xy[(i_max - 1) % n].0 == xy[i_max % n].0 {
        i_max -= 1;
    }
    for i in i_min..=i_max {
        v0.push(xy[i % n]);
    }

    let mut v1 = vec![]; // upper
    let i_min = pos_x_max;
    let mut i_max = pos_x_min + if pos_x_min < pos_x_max { n } else { 0 };
    if xy[(i_max - 1) % n].0 == xy[i_max % n].0 {
        i_max -= 1;
    }
    for i in (i_min..=i_max).rev() {
        v1.push(xy[i % n]);
    }

    (v0, v1)
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
        q: usize,
        ab: [(i64, i64); q],
    }

    let (v0, v1) = f(&xy); // (lower, upper)
    for &(a, b) in &ab {
        if a < v0[0].0 || a > v0[v0.len() - 1].0 {
            println!("{}", "OUT");
            continue;
        }
        if a == v0[0].0 {
            let on = v0[0].1 <= b && b <= v1[0].1;
            println!("{}", if on { "ON" } else { "OUT" });
            continue;
        }
        if a == v0[v0.len() - 1].0 {
            let on = v0[v0.len() - 1].1 <= b && b <= v1[v1.len() - 1].1;
            println!("{}", if on { "ON" } else { "OUT" });
            continue;
        }

        {
            let i0 = match v0.binary_search_by(|&xy| xy.0.cmp(&a)) {
                Ok(i) | Err(i) => i,
            };
            let x0 = v0[i0 - 1].0;
            let y0 = v0[i0 - 1].1;
            let x1 = v0[i0].0;
            let y1 = v0[i0].1;
            let z0 = (x1 - a) * y0 + (a - x0) * y1;
            let z = (x1 - x0) * b;
            if z == z0 {
                println!("{}", "ON");
                continue;
            } else if z < z0 {
                println!("{}", "OUT");
                continue;
            }
        }

        {
            let i0 = match v1.binary_search_by(|&xy| xy.0.cmp(&a)) {
                Ok(i) | Err(i) => i,
            };
            let x0 = v1[i0 - 1].0;
            let y0 = v1[i0 - 1].1;
            let x1 = v1[i0].0;
            let y1 = v1[i0].1;
            let z0 = (x1 - a) * y0 + (a - x0) * y1;
            let z = (x1 - x0) * b;
            if z == z0 {
                println!("{}", "ON");
                continue;
            } else if z > z0 {
                println!("{}", "OUT");
                continue;
            }
        }

        println!("{}", "IN");
    }
}
