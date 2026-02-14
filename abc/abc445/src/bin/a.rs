use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let yes = s[0] == s[s.len() - 1];
    println!("{}", if yes { "Yes" } else { "No" });
}
