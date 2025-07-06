use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        rc: [(Usize1, Usize1); k],
    }

    let mut m = vec![vec![usize::MAX; w]; h];
    let mut heap = BinaryHeap::new();
    for (r, c) in rc {
        m[r][c] = 0;
        if r > 0 {
            if (c > 0 && m[r - 1][c - 1] != usize::MAX)
                || (c + 1 < w && m[r - 1][c + 1] != usize::MAX)
                || (r - 1 > 0 && m[r - 2][c] != usize::MAX)
            {
                heap.push((Reverse(1), (r - 1, c)));
            }
        }
        if r + 1 < h {
            if (c > 0 && m[r + 1][c - 1] != usize::MAX)
                || (c + 1 < w && m[r + 1][c + 1] != usize::MAX)
                || (r + 2 < h && m[r + 2][c] != usize::MAX)
            {
                heap.push((Reverse(1), (r + 1, c)));
            }
        }
        if c > 0 {
            if (r > 0 && m[r - 1][c - 1] != usize::MAX)
                || (r + 1 < h && m[r + 1][c - 1] != usize::MAX)
                || (c - 1 > 0 && m[r][c - 2] != usize::MAX)
            {
                heap.push((Reverse(1), (r, c - 1)));
            }
        }
        if c + 1 < w {
            if (r > 0 && m[r - 1][c + 1] != usize::MAX)
                || (r + 1 < h && m[r + 1][c + 1] != usize::MAX)
                || (c + 2 < w && m[r][c + 2] != usize::MAX)
            {
                heap.push((Reverse(1), (r, c + 1)));
            }
        }
    }

    while let Some((Reverse(d), (r, c))) = heap.pop() {
        if m[r][c] <= d {
            continue;
        }
        m[r][c] = d;

        if r > 0 {
            if (c > 0 && m[r - 1][c - 1] != usize::MAX)
                || (c + 1 < w && m[r - 1][c + 1] != usize::MAX)
                || (r - 1 > 0 && m[r - 2][c] != usize::MAX)
            {
                heap.push((Reverse(d + 1), (r - 1, c)));
            }
        }
        if r + 1 < h {
            if (c > 0 && m[r + 1][c - 1] != usize::MAX)
                || (c + 1 < w && m[r + 1][c + 1] != usize::MAX)
                || (r + 2 < h && m[r + 2][c] != usize::MAX)
            {
                heap.push((Reverse(d + 1), (r + 1, c)));
            }
        }
        if c > 0 {
            if (r > 0 && m[r - 1][c - 1] != usize::MAX)
                || (r + 1 < h && m[r + 1][c - 1] != usize::MAX)
                || (c - 1 > 0 && m[r][c - 2] != usize::MAX)
            {
                heap.push((Reverse(d + 1), (r, c - 1)));
            }
        }
        if c + 1 < w {
            if (r > 0 && m[r - 1][c + 1] != usize::MAX)
                || (r + 1 < h && m[r + 1][c + 1] != usize::MAX)
                || (c + 2 < w && m[r][c + 2] != usize::MAX)
            {
                heap.push((Reverse(d + 1), (r, c + 1)));
            }
        }
    }

    let mut result = 0usize;
    for r in 0..h {
        for c in 0..w {
            if m[r][c] != usize::MAX {
                result += m[r][c];
            }
        }
    }
    println!("{result}");
}
