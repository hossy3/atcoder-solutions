use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let result = (0..(n / 2)).filter(|&i| s[i] != s[n - i - 1]).count();
    println!("{result}");
}
