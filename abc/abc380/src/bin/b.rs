use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut results = vec![];
    let v: Vec<_> = s.iter().positions(|&c| c == '|').collect();
    for v in v.windows(2) {
        results.push(v[1] - v[0] - 1);
    }
    let result = results.iter().join(" ");
    println!("{result}");
}
