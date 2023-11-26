use std::collections::HashSet;

use ac_library::Dsu;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut uf = Dsu::new(h * w);
    let mut set = HashSet::new();
    for r in 0..h {
        for c in 0..w {
            if s[r][c] != '#' {
                continue;
            }
            set.insert((r, c));
            let i = r * w + c;

            for dr in (-1)..=1 {
                for dc in (-1)..=1 {
                    let r0 = r.wrapping_add_signed(dr);
                    let c0 = c.wrapping_add_signed(dc);
                    if set.contains(&(r0, c0)) {
                        let i0 = r0 * w + c0;
                        uf.merge(i, i0);
                    }
                }
            }
        }
    }

    let mut set0 = HashSet::new();
    for &(r, c) in &set {
        set0.insert(uf.leader(r * w + c));
    }
    let result = set0.len();
    println!("{result}");
}
