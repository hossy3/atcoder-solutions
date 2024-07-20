use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        y: usize,
        a: [[Usize1; w]; h],
    }

    let mut queue = BinaryHeap::new();
    let mut set = HashSet::new();

    // 外周を登録する
    for i in 0..w {
        queue.push((Reverse(a[0][i]), 0, i));
        set.insert((a[0][i], 0, i));
        if h > 1 {
            queue.push((Reverse(a[h - 1][i]), h - 1, i));
            set.insert((a[h - 1][i], h - 1, i));
        }
    }
    for i in 1..(h - 1) {
        queue.push((Reverse(a[i][0]), i, 0));
        set.insert((a[i][0], i, 0));
        if w > 1 {
            queue.push((Reverse(a[i][w - 1]), i, w - 1));
            set.insert((a[i][w - 1], i, w - 1));
        }
    }

    let mut cum = vec![0; y];
    for y0 in 0..y {
        while let Some(&(Reverse(y1), i, j)) = queue.peek() {
            if y1 > y0 {
                break;
            }
            queue.pop();
            cum[y0] += 1;
            if i > 0 && !set.contains(&(a[i - 1][j], i - 1, j)) {
                queue.push((Reverse(a[i - 1][j]), i - 1, j));
                set.insert((a[i - 1][j], i - 1, j));
            }
            if i < h - 1 && !set.contains(&(a[i + 1][j], i + 1, j)) {
                queue.push((Reverse(a[i + 1][j]), i + 1, j));
                set.insert((a[i + 1][j], i + 1, j));
            }
            if j > 0 && !set.contains(&(a[i][j - 1], i, j - 1)) {
                queue.push((Reverse(a[i][j - 1]), i, j - 1));
                set.insert((a[i][j - 1], i, j - 1));
            }
            if j < w - 1 && !set.contains(&(a[i][j + 1], i, j + 1)) {
                queue.push((Reverse(a[i][j + 1]), i, j + 1));
                set.insert((a[i][j + 1], i, j + 1));
            }
        }
    }

    for y0 in 1..y {
        cum[y0] += cum[y0 - 1];
    }
    for y0 in 0..y {
        println!("{}", h * w - cum[y0]);
    }
}
