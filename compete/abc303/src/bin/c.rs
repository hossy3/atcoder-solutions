use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        m: usize,
        mut h: i64,
        k: i64,
        s: Chars,
        xy: [(i64, i64); m],
    }
    let mut set = HashSet::new();
    for &(x, y) in &xy {
        set.insert((x, y));
    }
    let mut x = 0;
    let mut y = 0;
    for &c in &s {
        h -= 1;
        if h < 0 {
            println!("{}", "No");
            return;
        }
        match c {
            'R' => {
                x += 1;
            }
            'L' => {
                x -= 1;
            }
            'U' => {
                y += 1;
            }
            'D' => {
                y -= 1;
            }
            _ => {
                panic!()
            }
        }
        if h < k && set.contains(&(x, y)) {
            h = k;
            set.remove(&(x, y));
        }
    }
    println!("{}", "Yes");
}
