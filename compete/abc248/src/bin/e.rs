use std::collections::HashMap;

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
        k: usize,
        xy: [(i64, i64); n],
    }

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut count = 0;
    for i in 0..(n - 1) {
        let mut m = HashMap::new();
        let (x0, y0) = xy[i];
        for j in (i + 1)..n {
            let (x1, y1) = xy[j];
            let mut x = x1 - x0;
            let mut y = y1 - y0;
            if x == 0 {
                y = 1;
            } else if y == 0 {
                x = 1;
            } else {
                let mut m = gcd(x.abs(), y.abs());
                if y < 0 {
                    m *= -1;
                }
                x /= m;
                y /= m;
            }
            *m.entry((x, y)).or_insert(0usize) += 1;
        }

        count += m.iter().filter(|&(_, &m)| m == k - 1).count();
    }

    println!("{}", count);
}
