extern crate nalgebra as na;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
        m: usize,
    }
    let mut ops = vec![na::Matrix3::identity(); m + 1];
    for i in 0..m {
        input! { t: usize }
        let op = match t {
            1 => na::Matrix3::new(0, -1, 0, 1, 0, 0, 0, 0, 1),
            2 => na::Matrix3::new(0, 1, 0, -1, 0, 0, 0, 0, 1),
            3 => {
                input! { p: i64 }
                na::Matrix3::new(-1, 0, 0, 0, 1, 0, p * 2, 0, 1)
            }
            4 => {
                input! { p: i64 }
                na::Matrix3::new(1, 0, 0, 0, -1, 0, 0, p * 2, 1)
            }
            _ => unreachable!(),
        };
        ops[i + 1] = ops[i] * op;
    }

    input! {
        q: usize,
        ab: [(usize, Usize1); q],
    }
    for &(a, b) in &ab {
        let v = na::RowVector3::new(xy[b].0, xy[b].1, 1);
        let result = v * ops[a];
        println!("{} {}", result[0], result[1]);
    }
}
