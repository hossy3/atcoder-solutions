use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut m = vec![vec![0usize; n]; n];
    let mut cur = 1;
    for i in 0..(n / 2) {
        for j in i..(n - i - 1) {
            m[i][j] = cur;
            cur += 1;
        }
        for j in i..(n - i - 1) {
            m[j][n - i - 1] = cur;
            cur += 1;
        }
        for j in ((i + 1)..(n - i)).rev() {
            m[n - i - 1][j] = cur;
            cur += 1;
        }
        for j in ((i + 1)..(n - i)).rev() {
            m[j][i] = cur;
            cur += 1;
        }
    }

    for i in 0..n {
        if i == n / 2 {
            let result0 = m[i][..(n / 2)].iter().join(" ");
            let result1 = m[i][(n / 2 + 1)..].iter().join(" ");
            println!("{result0} T {result1}");
        } else {
            let result = m[i].iter().join(" ");
            println!("{result}");
        }
    }
}
