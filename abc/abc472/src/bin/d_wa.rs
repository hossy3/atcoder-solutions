use std::{collections::VecDeque, eprintln};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    }

    let mut bomb_row = vec![false; h]; // 爆弾がある行
    let mut bomb_col = vec![false; w]; // 爆弾がある列
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                bomb_row[i] = true;
                bomb_col[j] = true;
            }
        }
    }

    let mut m = vec![vec![usize::MAX; w]; h]; // 何歩以内にたどり着くか
    let mut queue = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if !bomb_row[i] && !bomb_col[j] {
                m[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }
    eprintln!("{:?}", &queue);

    while let Some((i, j)) = queue.pop_front() {
        for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let step = m[i][j] + 1;
            let i = i.wrapping_add_signed(di);
            let j = j.wrapping_add_signed(dj);
            if i >= h || j >= w {
                continue;
            }
            if m[i][j] <= step {
                continue;
            }
            if s[i][j] == '#' {
                continue; // 爆弾マスには移動できない
            }
            m[i][j] = step;
            if step < k {
                queue.push_back((i, j));
            }
        }
    }

    let result = m
        .iter()
        .map(|row| row.iter().filter(|&&x| x < usize::MAX).count())
        .sum::<usize>();
    println!("{result}");
}
