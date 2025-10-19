use proconio::{input, marker::Chars};
use std::{cmp::Reverse, collections::BinaryHeap};

fn get_pos(a: &[Vec<char>], c: char) -> (usize, usize) {
    for (i, row) in a.iter().enumerate() {
        for (j, &c0) in row.iter().enumerate() {
            if c0 == c {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn main() {
    input! {
        (h, w): (usize, usize),
        a: [Chars; h],
    }

    let (rs, cs) = get_pos(&a, 'S');
    let (rt, ct) = get_pos(&a, 'G');

    let mut map = vec![vec![[usize::MAX, usize::MAX]; w]; h]; // switch off/on
    map[rs][cs] = [0, 0];
    let drc = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // UDLR

    let mut queue = BinaryHeap::<(Reverse<usize>, usize, usize, usize)>::new();
    queue.push((Reverse(0), rs, cs, 0));
    while let Some((Reverse(score), r, c, s)) = queue.pop() {
        if score > h * w * 2 {
            break;
        }
        for &(dr, dc) in &drc {
            let r0 = r.wrapping_add_signed(dr);
            let c0 = c.wrapping_add_signed(dc);
            if r0 >= h || c0 >= w {
                continue;
            }
            if a[r0][c0] == '#' || map[r0][c0][s] < usize::MAX {
                continue;
            }
            if (s == 0 && a[r0][c0] == 'x') || (s == 1 && a[r0][c0] == 'o') {
                continue;
            }
            let score = score + 1;
            map[r0][c0][s] = score;
            let s = if a[r0][c0] == '?' { 1 - s } else { s };
            queue.push((Reverse(score), r0, c0, s));
        }
    }

    let score = *map[rt][ct].iter().min().unwrap();
    if score < usize::MAX {
        println!("{score}");
    } else {
        println!("-1");
    }
}
