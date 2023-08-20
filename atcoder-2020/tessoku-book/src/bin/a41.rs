use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let yes = s.windows(3).any(|s| s[0] == s[1] && s[1] == s[2]);
    println!("{}", if yes { "Yes" } else { "No" });
}
