use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
        (a, b, c, d): (Usize1, Usize1, Usize1, Usize1),
    }

    let mut m = vec![vec![usize::MAX; w]; h];
    m[a][b] = 0;
    let drc = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut queue = BinaryHeap::<(Reverse<usize>, usize, usize)>::new();
    queue.push((Reverse(0), a, b));
    while let Some((_, r, c)) = queue.pop() {
        for &(dr, dc) in &drc {
            let score = m[r][c];
            let r0 = r.wrapping_add_signed(dr);
            let c0 = c.wrapping_add_signed(dc);
            if r0 >= h || c0 >= w {
                continue;
            }
            if s[r0][c0] == '.' && m[r0][c0] > score {
                m[r0][c0] = score;
                queue.push((Reverse(score), r0, c0));
            } else if s[r0][c0] == '#' && m[r0][c0] > score + 1 {
                m[r0][c0] = score + 1;
                queue.push((Reverse(score + 1), r0, c0));
            }

            let r1 = r0.wrapping_add_signed(dr);
            let c1 = c0.wrapping_add_signed(dc);
            if r1 >= h || c1 >= w {
                continue;
            }
            if m[r1][c1] > score + 1 {
                m[r1][c1] = score + 1;
                queue.push((Reverse(score + 1), r1, c1));
            }
        }
    }

    let score = m[c][d];
    println!("{score}");
}
