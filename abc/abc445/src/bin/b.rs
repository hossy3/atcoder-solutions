use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let m = s.iter().map(|s| s.len()).max().unwrap();
    for s in s {
        let pad = (vec!['.'; (m - s.len()) / 2]).iter().join("");
        let s = s.iter().join("");
        println!("{pad}{s}{pad}");
    }
}
