use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn f((x, y): (i128, i128)) -> (u8, i128) {
    const K: i128 = 1 << 96;
    if x == 0 {
        if y > 0 { (0, 0) } else { (2, 0) }
    } else if x > 0 {
        (1, -y * K / x)
    } else {
        (3, -y * K / x)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i128, i128); n],
        ab: [(Usize1, Usize1); q],
    }

    let mut v = vec![]; // (0右 1左..., 傾き)
    for &xy in &xy {
        v.push(f(xy));
    }
    v.sort();

    let mut results = vec![];
    for &(a, b) in &ab {
        let p0 = f(xy[a]);
        let p1 = f(xy[b]);
        let l = v.partition_point(|&p| p < p0);
        let r = v.partition_point(|&p| p <= p1);
        // eprintln!("{:?} {:?} {} {}", p0, p1, l, r);
        if p0 <= p1 {
            results.push(r - l);
        } else {
            results.push((n - l) + r);
        }
    }

    println!("{}", results.iter().join("\n"));
}
