use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    }
    let mut m = HashSet::<(usize, usize)>::new();
    let mut x: usize = 1;
    let mut y: usize = 1;
    loop {
        let x0 = x;
        let y0 = y;
        let c = g[y - 1][x - 1];
        match c {
            'U' => {
                y -= 1;
            }
            'D' => {
                y += 1;
            }
            'L' => {
                x -= 1;
            }
            'R' => {
                x += 1;
            }
            _ => {}
        }
        if x <= 0 || x > w || y <= 0 || y > h {
            println!("{} {}", y0, x0);
            return;
        }
        if m.contains(&(x, y)) {
            println!("-1");
            return;
        }
        m.insert((x, y));
    }
}
