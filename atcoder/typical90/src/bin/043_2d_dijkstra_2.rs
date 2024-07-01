use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        (h, w): (usize, usize),
        (rs, cs): (Usize1, Usize1),
        (rt, ct): (Usize1, Usize1),
        s: [Chars; h],
    }

    let mut a = vec![vec![[usize::MAX, usize::MAX, usize::MAX, usize::MAX]; w]; h]; // UDLR
    a[rs][cs] = [0, 0, 0, 0];
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut queue = BinaryHeap::<(Reverse<usize>, (usize, usize, usize))>::new(); // (cost, (r, c, dir))
    for dir in 0..4 {
        queue.push((Reverse(0), (rs, cs, dir)));
    }

    while let Some((Reverse(cost), (r, c, dir))) = queue.pop() {
        let (dr, dc) = dirs[dir];
        let r0 = r.wrapping_add_signed(dr);
        let c0 = c.wrapping_add_signed(dc);
        if r0 < h && c0 < w && s[r0][c0] != '#' {
            if a[r0][c0][dir] > cost {
                a[r0][c0][dir] = cost;
                queue.push((Reverse(cost), (r0, c0, dir)));
            }
        }

        let cost0 = cost + 1;
        for (dir0, _) in dirs.iter().enumerate() {
            if a[r][c][dir0] > cost0 {
                a[r][c][dir0] = cost0;
                queue.push((Reverse(cost0), (r, c, dir0)))
            }
        }
    }

    let score = a[rt][ct].iter().min().unwrap();
    println!("{score}");
}
