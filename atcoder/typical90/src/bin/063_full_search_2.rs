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

fn f(p: &[Vec<usize>], b: u8, j: usize) -> Option<usize> {
    let h = p.len();
    let mut val = None;
    for i in 0..h {
        if !b.bit_test(i) {
            continue;
        }
        let x = p[i][j];
        if val.is_some_and(|x0| x0 != x) {
            return None;
        }
        val = Some(x);
    }
    val
}

fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[usize; w]; h],
    }

    let mut score = 0usize;
    for b in 1usize..(1 << h) {
        let b = b as u8;
        let row_count = b.count_ones() as usize;

        let mut m = HashMap::<usize, usize>::new();
        for j in 0..w {
            if let Some(max) = f(&p, b, j) {
                *m.entry(max).or_insert(0) += 1;
            }
        }
        let col_count = m.values().max().unwrap_or(&0);

        let s = row_count * col_count;
        score = score.max(s);
    }
    println!("{score}");
}
