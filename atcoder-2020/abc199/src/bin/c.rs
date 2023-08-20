use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        tab: [(usize, usize, usize); q],
    }

    let mut flip = false;
    let f = |i, flip| if flip { (i + n - 1) % (2 * n) } else { i - 1 };

    for &(t, a, b) in &tab {
        if t == 1 {
            s.swap(f(a, flip), f(b, flip));
        } else {
            flip = !flip;
        }
    }

    if flip {
        println!("{}{}", s[n..].iter().join(""), s[..n].iter().join(""));
    } else {
        println!("{}", s.iter().join(""));
    }
}
