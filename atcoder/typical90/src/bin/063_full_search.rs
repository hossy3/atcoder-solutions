use proconio::input;
use std::collections::HashMap;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for u8 {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn f(t: &[usize], b: u8) -> Option<[usize; 8]> {
    let mut a = [0usize; 8]; // t は 1 以上
    for i in 0..8 {
        if b.bit_test(i) {
            a[i] = t[i];
        }
    }
    let Some(&max) = a.iter().max() else { unreachable!() };
    if a.iter().all(|&x| x == 0 || x == max) {
        Some(a)
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
        let mut m = HashMap::<[usize; 8], usize>::new();
        for t in &t {
            if let Some(a8) = f(t, b) {
                *m.entry(a8).or_insert(0) += 1;
            }
        }
        let s = m.values().max().unwrap_or(&0) * c as usize;
        score = score.max(s);
    }
    println!("{score}");
}
