use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        ab: [(Usize1, Usize1); n],
    }

    let mut m = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            m[i][j] = (h - i).min(w - j);
        }
    }

    let mut queue = BinaryHeap::new();
    for &(a, b) in &ab {
        queue.push((Reverse(0), a, b));
        m[a][b] = 0;
    }
    while let Some((Reverse(len), i, j)) = queue.pop() {
        let len = len + 1;
        if i > 0 && m[i - 1][j] > len {
            queue.push((Reverse(len), i - 1, j));
            m[i - 1][j] = len;
        }
        if j > 0 && m[i][j - 1] > len {
            queue.push((Reverse(len), i, j - 1));
            m[i][j - 1] = len;
        }
        if i > 0 && j > 0 && m[i - 1][j - 1] > len {
            queue.push((Reverse(len), i - 1, j - 1));
            m[i - 1][j - 1] = len;
        }
    }

    let result: usize = m.iter().map(|v| v.iter().sum::<usize>()).sum();
    println!("{}", result);
}
