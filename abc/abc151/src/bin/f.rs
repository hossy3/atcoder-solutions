use itertools::Itertools;
use proconio::input;

const EPS: f64 = 1e-9;

fn dist2((x0, y0): (f64, f64), (x1, y1): (f64, f64)) -> f64 {
    (x0 - x1).powi(2) + (y0 - y1).powi(2)
}

// 2点または3点に接する円の中心
fn center(v: &[(f64, f64)]) -> (f64, f64) {
    let n = v.len();
    assert!(n == 2 || n == 3);

    if n == 2 {
        let (x0, y0) = v[0];
        let (x1, y1) = v[1];
        let cx = (x0 + x1) / 2.0;
        let cy = (y0 + y1) / 2.0;
        (cx, cy)
    } else {
        let (x0, y0) = v[0];
        let (x1, y1) = v[1];
        let (x2, y2) = v[2];

        let denom = 2.0 * ((x0 * y1 + x1 * y2 + x2 * y0) - (x0 * y2 + x1 * y0 + x2 * y1));
        if denom.abs() < EPS {
            let d01 = dist2(v[0], v[1]);
            let d12 = dist2(v[1], v[2]);
            let d20 = dist2(v[2], v[0]);
            if d01 > d12 && d01 > d20 {
                return center(&[v[0], v[1]]);
            } else if d12 > d01 && d12 > d20 {
                return center(&[v[1], v[2]]);
            } else {
                return center(&[v[2], v[0]]);
            }
        }
        let cx = ((x0.powi(2) + y0.powi(2)) * (y1 - y2)
            + (x1.powi(2) + y1.powi(2)) * (y2 - y0)
            + (x2.powi(2) + y2.powi(2)) * (y0 - y1))
            / denom;
        let cy = ((x0.powi(2) + y0.powi(2)) * (x2 - x1)
            + (x1.powi(2) + y1.powi(2)) * (x0 - x2)
            + (x2.powi(2) + y2.powi(2)) * (x1 - x0))
            / denom;
        (cx, cy)
    }
}

fn f(v: &mut Vec<(f64, f64)>) -> f64 {
    let n = v.len();
    if n < 2 {
        return 0.0;
    }

    // 2つの点の中点に他の全ての点が含まれるなら、それが正解
    'outer2: for a in (0..n).combinations(2) {
        let (xy0, xy1) = (v[a[0]], v[a[1]]);
        let xyc = center(&[xy0, xy1]);
        let r = dist2(xy0, xyc).sqrt();

        for i in 0..n {
            if i == a[0] || i == a[1] {
                continue;
            }
            let xy = v[i];
            if dist2(xy, xyc).sqrt() > r + EPS {
                continue 'outer2;
            }
        }

        *v = vec![xy0, xy1];
        return r;
    }

    // 3つの点を通る円に他の全ての点が含まれるなら、それが正解
    let mut cand_r = f64::MAX;
    let mut cand_v = vec![];
    'outer3: for a in (0..n).combinations(3) {
        let (xy0, xy1, xy2) = (v[a[0]], v[a[1]], v[a[2]]);
        let xyc = center(&[xy0, xy1, xy2]);
        let r = dist2(xy0, xyc).sqrt();
        if r > cand_r {
            continue;
        }

        for i in 0..n {
            if i == a[0] || i == a[1] || i == a[2] {
                continue;
            }
            let xy = v[i];
            if dist2(xy, xyc).sqrt() > r + EPS {
                continue 'outer3;
            }
        }

        cand_r = r;
        cand_v = vec![xy0, xy1, xy2];
    }

    *v = cand_v;
    cand_r
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_3() {
        let mut rng = rand::thread_rng();
        for k in 2..6 {
            for _ in 0..100 {
                let mut v = vec![];
                while v.len() < k {
                    let x = rng.gen_range(0..=1000) as f64;
                    let y = rng.gen_range(0..=1000) as f64;
                    if !v.contains(&(x, y)) {
                        v.push((x, y));
                    }
                }
                let r = f(&mut v);
                if v.len() > 3 {
                    panic!("Invalid circle: {:?}", &v);
                }
                let cxy = center(&v);
                for &(x, y) in &v {
                    let d = dist2((x as f64, y as f64), cxy).sqrt();
                    if d > r + EPS {
                        panic!("Invalid circle: {:?}, {:?}, {}", &v, cxy, r);
                    }
                }
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        mut xy: [(f64, f64); n],
    }

    let result = f(&mut xy);
    println!("{result}");

    // let mut v = vec![xy[0], xy[1]];
    // for &xy in &xy[2..] {
    //     v.push(xy);
    //     f(&mut v);
    // }

    // let xyc = center(&v);
    // let result = dist2(xy[0], xyc).sqrt();
    // println!("{result}");
}
