use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    for i in 0..n {
        for j in 0..n {
            let c = if i == 0 && j > 0 {
                a[i][j - 1]
            } else if j == n - 1 {
                a[i - 1][j]
            } else if i == n - 1 {
                a[i][j + 1]
            } else if j == 0 {
                a[i + 1][j]
            } else {
                a[i][j]
            };
            print!("{}", c);
        }
        println!();
    }
}
