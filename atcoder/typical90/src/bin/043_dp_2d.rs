use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: Usize1,
        cs: Usize1,
        rt: Usize1,
        ct: Usize1,
        s: [Chars; h],
    }

    let mut a = vec![vec![[usize::MAX, usize::MAX, usize::MAX, usize::MAX]; w + 2]; h + 2]; // UDLR
    a[rs][cs] = [0, 0, 0, 0];
    let drc = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut queue = BinaryHeap::<(Reverse<usize>, usize, usize)>::new();
    queue.push((Reverse(0), rs, cs));
    while let Some((_, r, c)) = queue.pop() {
        for (i, &(dr, dc)) in drc.iter().enumerate() {
            let score = a[r][c][i];
            let r0 = r.wrapping_add_signed(dr);
            let c0 = c.wrapping_add_signed(dc);
            if r0 >= h || c0 >= w {
                continue;
            }
            if s[r0][c0] == '#' || a[r0][c0][i] <= score {
                continue;
            }
            for j in 0..4 {
                if j == i {
                    a[r0][c0][j] = score;
                } else {
                    a[r0][c0][j] = a[r0][c0][j].min(score + 1);
                }
            }
            queue.push((Reverse(score), r0, c0));
        }
    }

    let score = a[rt][ct].iter().min().unwrap();
    println!("{score}");
}
