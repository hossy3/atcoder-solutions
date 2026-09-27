use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Usize1; n],
    }

    let mut p0 = p.clone();
    let mut p1 = vec![0usize; n];
    for (i, &x) in p.iter().enumerate() {
        p1[x] = i;
    }

    let mut b = true; // p 側を使っているか

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                }
                if b {
                    p0.swap(x, y);
                    p1.swap(p0[x], p0[y]);
                } else {
                    p1.swap(x, y);
                    p0.swap(p1[x], p1[y]);
                }
            }
            2 => {
                b = !b;
            }
            _ => unreachable!(),
        }
    }

    if b {
        println!("{}", p0.iter().map(|&x| x + 1).join(" "));
    } else {
        println!("{}", p1.iter().map(|&x| x + 1).join(" "));
    }
}
