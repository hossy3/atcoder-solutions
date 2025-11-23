use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    }

    let mut queue = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'E' {
                queue.push_back((i, j));
            }
        }
    }

    let dirs = [('v', -1, 0), ('^', 1, 0), ('>', 0, -1), ('<', 0, 1)];

    while let Some((i, j)) = queue.pop_front() {
        for &(c, di, dj) in &dirs {
            let i0 = i.wrapping_add_signed(di);
            let j0 = j.wrapping_add_signed(dj);
            if i0 < h && j0 < w && s[i0][j0] == '.' {
                s[i0][j0] = c;
                queue.push_back((i0, j0));
            }
        }
    }

    for s in &s {
        let result = s.iter().join("");
        println!("{result}");
    }
}
