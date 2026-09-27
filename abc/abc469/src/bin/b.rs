use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let result = (0..n)
        .filter(|&i| s[i] == 'x' && (i == 0 || s[i - 1] == 'x') && (i == n - 1 || s[i + 1] == 'x'))
        .count();
    println!("{result}");
}
