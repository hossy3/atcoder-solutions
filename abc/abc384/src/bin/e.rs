use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        p: Usize1,
        q: Usize1,
        s: [[usize; w]; h],
    }

    let mut b = vec![vec![false; w]; h]; // 高橋くんの場所
    b[p][q] = true;
    let mut power = s[p][q]; // 高橋くんの強さ

    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut candidates = BTreeSet::new();
    for dir in 0..4 {
        let r = p.wrapping_add_signed(dirs[dir].0);
        let c = q.wrapping_add_signed(dirs[dir].1);
        if r < h && c < w && !b[r][c] {
            candidates.insert((s[r][c], r, c));
        }
    }

    while let Some((power0, r, c)) = candidates.pop_first() {
        if (power0 as i128) * (x as i128) < (power as i128) {
            b[r][c] = true;
            power += s[r][c];
            for dir in 0..4 {
                let r = r.wrapping_add_signed(dirs[dir].0);
                let c = c.wrapping_add_signed(dirs[dir].1);
                if r < h && c < w && !b[r][c] {
                    candidates.insert((s[r][c], r, c));
                }
            }
        }
    }

    println!("{power}");
}
