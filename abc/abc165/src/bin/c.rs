use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcd: [(Usize1, Usize1, usize, usize); q],
    }

    let mut max = 0;
    for v in (1..=m).combinations_with_replacement(n) {
        let mut value = 0;
        for &(a, b, c, d) in &abcd {
            if v[b] - v[a] == c {
                value += d;
            }
        }
        max = max.max(value);
    }
    println!("{max}");
}
