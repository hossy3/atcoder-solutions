use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h],
    }

    let mut heap = BinaryHeap::new();
    let mut map = vec![vec![usize::MAX; w]; h];
    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for r in 0..h {
        for c in 0..w {
            if s[r][c] == 'H' {
                map[r][c] = 0;
                heap.push((Reverse(0usize), r, c));
            }
        }
    }

    while let Some((Reverse(score), r, c)) = heap.pop() {
        for dir in 0..4 {
            let r = r.wrapping_add_signed(dirs[dir].0);
            let c = c.wrapping_add_signed(dirs[dir].1);
            let score = score + 1;
            if r >= h || c >= w {
                continue;
            }
            if s[r][c] == '#' {
                continue;
            }

            if map[r][c] <= score {
                continue;
            }
            map[r][c] = score;
            heap.push((Reverse(score), r, c));
        }
    }

    let mut result = 0usize;
    for r in 0..h {
        for c in 0..w {
            if map[r][c] <= d {
                result += 1;
            }
        }
    }
    println!("{result}");
}
