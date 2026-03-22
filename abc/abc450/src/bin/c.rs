use std::collections::HashSet;

use ac_library::Dsu;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let outer = h * w;
    let f = |i: usize, j: usize| -> usize { i * w + j };

    let mut dsu = Dsu::new(h * w + 1);
    for i in 0..h {
        for j in 0..(w - 1) {
            if c[i][j] == '.' && c[i][j + 1] == '.' {
                dsu.merge(f(i, j), f(i, j + 1));
            }
        }
    }
    for i in 0..(h - 1) {
        for j in 0..w {
            if c[i][j] == '.' && c[i + 1][j] == '.' {
                dsu.merge(f(i, j), f(i + 1, j));
            }
        }
    }

    // 外周
    for i in 0..h {
        if c[i][0] == '.' {
            dsu.merge(outer, f(i, 0));
        }
        if c[i][w - 1] == '.' {
            dsu.merge(outer, f(i, w - 1));
        }
    }
    for j in 0..w {
        if c[0][j] == '.' {
            dsu.merge(outer, f(0, j));
        }
        if c[h - 1][j] == '.' {
            dsu.merge(outer, f(h - 1, j));
        }
    }

    let mut set = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '.' {
                set.insert(dsu.leader(f(i, j)));
            }
        }
    }
    set.remove(&dsu.leader(outer));

    let result = set.len();
    println!("{result}");
}
