use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut scores = vec![0; n];
    for i in 0..m {
        let b = (0..n).filter(|&j| s[j][i] == '0').count() < (0..n).filter(|&j| s[j][i] == '1').count();
        for j in 0..n {
            if (b && s[j][i] == '0') || (!b && s[j][i] == '1') {
                scores[j] += 1;
            }
        }
    }

    let max = *scores.iter().max().unwrap();
    let mut results = vec![];
    for (i, &x) in scores.iter().enumerate() {
        if x == max {
            results.push(i + 1);
        }
    }
    println!("{}", results.iter().join(" "));
}
