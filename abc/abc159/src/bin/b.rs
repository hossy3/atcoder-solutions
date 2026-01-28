use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let yes = (0..(n / 2)).all(|i| s[i] == s[n - i - 1])
        && (0..(n / 4)).all(|i| s[i] == s[n / 2 - i - 1])
        && (0..(n / 4)).all(|i| s[n / 2 + i + 1] == s[n - i - 1]);
    println!("{}", if yes { "Yes" } else { "No" });
}
