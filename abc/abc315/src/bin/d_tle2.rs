use std::{collections::{HashMap, HashSet}, mem::swap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [Chars; h],
    }

    let mut rows = vec![HashMap::new(); h];
    let mut cols = vec![HashMap::new(); w];

    for i in 0..h {
        for j in 0..w {
            let x = c[i][j];
            if x == '.' {
                continue;
            }
            rows[i].entry(x).or_insert(HashSet::new()).insert(j);
            cols[j].entry(x).or_insert(HashSet::new()).insert(i);
        }
    }

    let mut next_rows = HashSet::new();
    let mut next_cols = HashSet::new();
    for i in 0..h {
        next_rows.insert(i);
    }
    for j in 0..w {
        next_cols.insert(j);
    }

    loop {
        let mut cur_rows = HashSet::new();
        let mut cur_cols = HashSet::new();
        swap(&mut cur_rows, &mut next_rows);
        swap(&mut cur_cols, &mut next_cols);

        let mut modified = HashSet::new();
        for i in cur_rows {
            if rows[i].len() != 1 {
                continue;
            }
            let (x, js) = rows[i].iter().next().unwrap();
            if js.len() < 2 {
                continue;
            }
            next_rows.insert(i);
            for &j in js {
                c[i][j] = '.';
                modified.insert((i, j, *x));
                next_cols.insert(j);
            }
            rows[i].clear();
        }

        for j in cur_cols {
            if cols[j].len() != 1 {
                continue;
            }
            let (x, is) = cols[j].iter().next().unwrap();
            if is.len() < 2 {
                continue;
            }
            next_cols.insert(j);
            for &i in is {
                c[i][j] = '.';
                modified.insert((i, j, *x));
                next_rows.insert(i);
            }
            cols[j].clear();
        }

        if modified.len() == 0 {
            break;
        }

        for &(i, j, x) in &modified {
            if let Some(y) = rows[i].get_mut(&x) {
                if y.len() > 1 {
                    y.remove(&j);
                } else {
                    rows[i].remove(&x);
                }
            }

            if let Some(y) = cols[j].get_mut(&x) {
                if y.len() > 1 {
                    y.remove(&i);
                } else {
                    cols[j].remove(&x);
                }
            }
        }
    }

    let result: usize = c
        .iter()
        .map(|v| v.iter().filter(|&&x| x != '.').count())
        .sum();
    println!("{result}");
}
