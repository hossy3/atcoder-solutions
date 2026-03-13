use std::mem::swap;

use itertools::Itertools;
use proconio::input;
use rand::Rng;

fn hilbert_order(mut x: usize, mut y: usize) -> usize {
    const MAX: usize = 1 << 25;

    let mut d = 0;
    let mut s = MAX >> 1;
    while s > 0 {
        let rx = (x & s) > 0;
        let ry = (y & s) > 0;
        let r = (if rx { 3 } else { 0 }) ^ (if ry { 1 } else { 0 });
        d += s * s * r;
        s = s >> 1;
        if ry {
            continue;
        }
        if rx {
            x = MAX - 1 - x;
            y = MAX - 1 - y;
        }
        swap(&mut x, &mut y);
    }
    d
}

fn check(xy: &[(usize, usize)], v: &[usize]) -> bool {
    let n = xy.len();
    let mut sum = 0usize;
    for i in 0..n {
        let (i, j) = (v[i], v[(i + 1) % n]);
        sum += xy[i].0.abs_diff(xy[j].0) + xy[i].1.abs_diff(xy[j].1);
    }
    sum <= 10_000_000_000
}

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    let mut rng = rand::thread_rng();
    loop {
        let offset_x = rng.gen_range(0..((1 << 25) - 20_000_000));
        let offset_y = rng.gen_range(0..((1 << 25) - 20_000_000));
        let v = (0..n)
            .sorted_by_cached_key(|&i| hilbert_order(xy[i].0 + offset_x, xy[i].1 + offset_y))
            .collect::<Vec<_>>();
        let i0 = v.iter().position(|&i| i == 0).unwrap();
        let mut v0 = vec![];
        for i in i0..n {
            v0.push(v[i]);
        }
        for i in 0..i0 {
            v0.push(v[i]);
        }

        if check(&xy, &v0) {
            println!("{}", v0.iter().map(|&i| i + 1).join(" "));
            return;
        }
    }
}
