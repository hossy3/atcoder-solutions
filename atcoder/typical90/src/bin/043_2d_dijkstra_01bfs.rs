use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

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

    let mut queue = VecDeque::<((usize, usize, usize), usize)>::new(); // ((r, c, dir), cost)
    for dir in 0..4 {
        queue.push_back(((rs, cs, dir), 0));
    }

    while let Some(((r, c, dir), cost)) = queue.pop_front() {
        let (dr, dc) = dirs[dir];
        let r0 = r.wrapping_add_signed(dr);
        let c0 = c.wrapping_add_signed(dc);
        if r0 < h && c0 < w && s[r0][c0] != '#' {
            if a[r0][c0][dir] > cost {
                a[r0][c0][dir] = cost;
                queue.push_front(((r0, c0, dir), cost));
            }
        }

        let cost0 = cost + 1;
        for (dir0, _) in dirs.iter().enumerate() {
            if dir0 / 2 != dir / 2 {
                if a[r][c][dir0] > cost0 {
                    a[r][c][dir0] = cost0;
                    queue.push_back(((r, c, dir0), cost0))
                }
            }
        }
    }

    let score = a[rt][ct].iter().min().unwrap();
    println!("{score}");
}
