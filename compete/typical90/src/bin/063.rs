use std::collections::HashMap;

use proconio::input;

fn f(t: &[usize], b: u8) -> Option<(usize, usize, usize, usize, usize, usize, usize, usize)> {
    let x = (
        if b & 0b00000001 > 0 { t[0] } else { 0 },
        if b & 0b00000010 > 0 { t[1] } else { 0 },
        if b & 0b00000100 > 0 { t[2] } else { 0 },
        if b & 0b00001000 > 0 { t[3] } else { 0 },
        if b & 0b00010000 > 0 { t[4] } else { 0 },
        if b & 0b00100000 > 0 { t[5] } else { 0 },
        if b & 0b01000000 > 0 { t[6] } else { 0 },
        if b & 0b10000000 > 0 { t[7] } else { 0 },
    );
    let a = [x.0, x.1, x.2, x.3, x.4, x.5, x.6, x.7];
    let &max = a.iter().max().unwrap_or(&0);
    if a.iter().all(|&x| x == 0 || x == max) {
        Some(x)
    } else {
        None
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[usize; w]; h],
    }

    let mut t = vec![vec![0usize; h]; w];
    for i in 0..h {
        for j in 0..w {
            t[j][i] = p[i][j];
        }
    }

    let mut score = 0usize;
    let bmax = ((1 << h) - 1) as u8;
    for b in 1..=bmax {
        let c = b.count_ones();
        let mut m =
            HashMap::<(usize, usize, usize, usize, usize, usize, usize, usize), usize>::new();
        for t in &t {
            if let Some(tuple8) = f(t, b) {
                if let Some(x) = m.get_mut(&tuple8) {
                    *x += 1;
                } else {
                    m.insert(tuple8, 1);
                }
            }
        }
        let s = m.values().max().unwrap_or(&0) * c as usize;
        score = score.max(s);
    }
    println!("{}", score);
}
