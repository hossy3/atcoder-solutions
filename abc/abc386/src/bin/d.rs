use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn f(n: usize, x2yc: &BTreeSet<(usize, usize, char)>) -> bool {
    let mut y0 = n; // 白がはじまる場所
    for &(_, y, c) in x2yc {
        match c {
            'B' => {
                if y >= y0 {
                    return false;
                }
            }
            'W' => {
                y0 = y0.min(y);
            }
            _ => unreachable!(),
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xyc: [(Usize1, Usize1, char); m],
    }

    let mut x2yc = BTreeSet::new();
    let mut y2xc = BTreeSet::new();
    for &(x, y, c) in &xyc {
        x2yc.insert((x, y, c));
        y2xc.insert((y, x, c));
    }
    let yes = f(n, &x2yc) && f(n, &y2xc);
    println!("{}", if yes { "Yes" } else { "No" });
}
