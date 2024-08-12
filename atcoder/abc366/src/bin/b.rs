use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let m = s.iter().map(|s| s.len()).max().unwrap();
    let mut mat = vec![vec!['*'; n]; m];
    for (i, s) in s.iter().enumerate() {
        for (j, &c) in s.iter().enumerate() {
            mat[j][n - i - 1] = c;
        }
    }

    for i in 0..mat.len() {
        while let Some(&c) = mat[i].last() {
            if c != '*' {
                break;
            }
            mat[i].pop();
        }

        let result = mat[i].iter().join("");
        println!("{result}");
    }
}
