use std::collections::HashSet;

use proconio::input;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut set = HashSet::new();
    for (i, &(x0, y0)) in xy.iter().enumerate() {
        for &(x1, y1) in &xy[(i + 1)..] {
            let (x, y) = if x0 == x1 {
                (0, 1)
            } else if y0 == y1 {
                (1, 0)
            } else {
                let x = x1 - x0;
                let y = y1 - y0;
                let k = gcd(x.abs(), y.abs());
                (x / k, y / k)
            };
            set.insert((x, y));
            set.insert((-x, -y));
        }
    }

    let result = set.len();
    println!("{}", result);
}
