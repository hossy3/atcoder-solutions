use std::{collections::HashSet, mem::swap};

use proconio::{input, marker::Chars};

fn f(c: char) -> usize {
    c as usize - 'a' as usize
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [Chars; h],
    }

    let mut rows = vec![vec![0; 26]; h];
    let mut cols = vec![vec![0; 26]; w];

    for i in 0..h {
        for j in 0..w {
            let x = f(c[i][j]);
            rows[i][x] += 1;
            cols[j][x] += 1;
        }
    }

    let mut next_rows = vec![true; h];
    let mut next_cols = vec![true; w];

    loop {
        let mut cur_rows = vec![false; h];
        let mut cur_cols = vec![false; w];
        swap(&mut cur_rows, &mut next_rows);
        swap(&mut cur_cols, &mut next_cols);

        let mut modified = HashSet::new();
        for (i, b) in cur_rows.iter().enumerate() {
            if !b {
                continue;
            }
            if rows[i].iter().filter(|&&x| x > 0).count() != 1 {
                continue;
            }
            let x = rows[i].iter().position(|&x| x > 0).unwrap();
            if rows[i][x] < 2 {
                continue;
            }
            for j in 0..w {
                if f(c[i][j]) == x {
                    modified.insert((i, j, x));
                }
            }
        }

        for (j, b) in cur_cols.iter().enumerate() {
            if !b {
                continue;
            }
            if cols[j].iter().filter(|&&x| x > 0).count() != 1 {
                continue;
            }
            let x = cols[j].iter().position(|&x| x > 0).unwrap();
            if cols[j][x] < 2 {
                continue;
            }
            for i in 0..h {
                if f(c[i][j]) == x {
                    modified.insert((i, j, x));
                }
            }
        }

        if modified.len() == 0 {
            break;
        }

        for &(i, j, x) in &modified {
            rows[i][x] -= 1;
            cols[j][x] -= 1;
            next_rows[i] = true;
            next_cols[j] = true;
        }
    }

    let result: usize = rows.iter().map(|v| v.iter().sum::<usize>()).sum();
    println!("{result}");
}
