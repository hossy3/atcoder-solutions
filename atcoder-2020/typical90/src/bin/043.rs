use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Chars};

fn init_s(h: usize, w: usize, s: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut a = vec![vec![false; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            a[i + 1][j + 1] = s[i][j] == '.';
        }
    }
    a
}

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: usize,
        cs: usize,
        rt: usize,
        ct: usize,
        s: [Chars; h],
    }

    let s = init_s(h, w, &s);

    const MAX: usize = 1_usize << 60;
    let mut a = vec![vec![(MAX, MAX, MAX, MAX); w + 2]; h + 2]; // UDLR
    a[rs][cs] = (0, 0, 0, 0);
    let mut queue = BinaryHeap::<(Reverse<usize>, usize, usize)>::new();
    queue.push((Reverse(0), rs, cs));
    while let Some((_, r, c)) = queue.pop() {
        if s[r - 1][c] && a[r - 1][c].0 > a[r][c].0 {
            a[r - 1][c] = (
                a[r][c].0,
                a[r - 1][c].1.min(a[r][c].0 + 1),
                a[r - 1][c].2.min(a[r][c].0 + 1),
                a[r - 1][c].3.min(a[r][c].0 + 1),
            );
            queue.push((Reverse(a[r][c].0), r - 1, c));
        }

        if s[r + 1][c] && a[r + 1][c].1 > a[r][c].1 {
            a[r + 1][c] = (
                a[r + 1][c].0.min(a[r][c].1 + 1),
                a[r][c].1,
                a[r + 1][c].2.min(a[r][c].1 + 1),
                a[r + 1][c].3.min(a[r][c].1 + 1),
            );
            queue.push((Reverse(a[r][c].1), r + 1, c));
        }

        if s[r][c - 1] && a[r][c - 1].2 > a[r][c].2 {
            a[r][c - 1] = (
                a[r][c - 1].0.min(a[r][c].2 + 1),
                a[r][c - 1].1.min(a[r][c].2 + 1),
                a[r][c].2,
                a[r][c - 1].3.min(a[r][c].2 + 1),
            );
            queue.push((Reverse(a[r][c].2), r, c - 1));
        }

        if s[r][c + 1] && a[r][c + 1].3 > a[r][c].3 {
            a[r][c + 1] = (
                a[r][c + 1].0.min(a[r][c].3 + 1),
                a[r][c + 1].1.min(a[r][c].3 + 1),
                a[r][c + 1].2.min(a[r][c].3 + 1),
                a[r][c].3,
            );
            queue.push((Reverse(a[r][c].2), r, c + 1));
        }
    }

    let score = a[rt][ct]
        .0
        .min(a[rt][ct].1)
        .min(a[rt][ct].2)
        .min(a[rt][ct].3);
    println!("{}", score);
}
