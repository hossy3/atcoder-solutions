use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let yes = (0..n).all(|i| s[i] == t[i] || t[i] == '*');
    println!("{}", if yes { "Yes" } else { "No" });
}
