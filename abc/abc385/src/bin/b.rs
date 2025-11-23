use std::collections::BTreeSet;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        _w: usize,
        mut x: Usize1,
        mut y: Usize1,
        s: [Chars; h],
        t: Chars,
    }

    let mut set = BTreeSet::new();
    for c in t {
        let (dx, dy) = match c {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => unreachable!(),
        };
        let x0 = x.wrapping_add_signed(dx);
        let y0 = y.wrapping_add_signed(dy);
        if s[x0][y0] == '#' {
            continue;
        }
        (x, y) = (x0, y0);
        if s[x][y] == '@' {
            set.insert((x, y));
        }
    }

    let (x, y, result) = (x + 1, y + 1, set.len());
    println!("{x} {y} {result}");
}
