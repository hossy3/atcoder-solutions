use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut v = vec![vec![false; n]; n];
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let b = (0..m).filter(|&k| s[i][k] == s[j][k]).count() == m - 1;
            v[i][j] = b;
            v[j][i] = b;
        }
    }
    let yes = (0..n)
        .permutations(n)
        .any(|a| a.windows(2).all(|x| v[x[0]][x[1]]));
    println!("{}", if yes { "Yes" } else { "No" });
}
