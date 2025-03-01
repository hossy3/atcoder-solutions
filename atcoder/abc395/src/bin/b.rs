use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![vec!['.'; n]; n];
    for i in 0..=(n / 2) {
        let x = if i % 2 == 0 { '#' } else { '.' };
        for r in i..(n - i) {
            for c in i..(n - i) {
                v[r][c] = x;
            }
        }
    }

    for v in v {
        println!("{}", v.iter().join(""));
    }
}
