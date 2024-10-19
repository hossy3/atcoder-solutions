use itertools::Itertools;
use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn distance(a: f64, b: f64, c: f64, d: f64) -> f64 {
    ((c - a).powi(2) + (d - b).powi(2)).sqrt()
}

fn f(abcd: &[(f64, f64, f64, f64)], permutations: &[usize], dirs: usize) -> f64 {
    let mut result = 0.0;
    for i in 0..(abcd.len()) {
        let (x0, y0) = if i == 0 {
            (0.0, 0.0)
        } else {
            let j = permutations[i - 1];
            if dirs.bit_test(j) {
                (abcd[j].0, abcd[j].1)
            } else {
                (abcd[j].2, abcd[j].3)
            }
        };
        let j = permutations[i];
        let (x1, y1) = if dirs.bit_test(j) {
            (abcd[j].2, abcd[j].3)
        } else {
            (abcd[j].0, abcd[j].1)
        };
        result += distance(x0, y0, x1, y1);
    }
    result
}

fn main() {
    input! {
        n: usize,
        s: f64,
        t: f64,
        abcd: [(f64, f64, f64, f64); n],
    }

    let mut result = f64::MAX;
    for permutations in (0..n).permutations(n) {
        for i in 0..(1 << n) {
            result = result.min(f(&abcd, &permutations, i) / s);
        }
    }
    eprintln!("{result}");

    for &(a, b, c, d) in &abcd {
        result += distance(a, b, c, d) / t;
    }

    println!("{result}");
}
