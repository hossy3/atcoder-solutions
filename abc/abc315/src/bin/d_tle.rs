use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [Chars; h],
    }

    loop {
        let mut rows = vec![HashMap::new(); h];
        let mut cols = vec![HashMap::new(); w];

        for i in 0..h {
            for j in 0..w {
                let x = c[i][j];
                if x == '.' {
                    continue;
                }
                rows[i].entry(x).or_insert(vec![]).push(j);
                cols[j].entry(x).or_insert(vec![]).push(i);
            }
        }

        let mut modified = false;
        for i in 0..h {
            if rows[i].len() != 1 {
                continue;
            }
            let (_, js) = rows[i].iter().next().unwrap();
            if js.len() < 2 {
                continue;
            }
            modified = true;
            for &j in js {
                c[i][j] = '.';
            }
        }

        for j in 0..w {
            if cols[j].len() != 1 {
                continue;
            }
            let (_, is) = cols[j].iter().next().unwrap();
            if is.len() < 2 {
                continue;
            }
            modified = true;
            for &i in is {
                c[i][j] = '.';
            }
        }

        if !modified {
            break;
        }
    }

    let result: usize = c
        .iter()
        .map(|v| v.iter().filter(|&&x| x != '.').count())
        .sum();
    println!("{result}");
}
