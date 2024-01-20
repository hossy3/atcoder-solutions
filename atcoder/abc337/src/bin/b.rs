use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut i = s.iter().position(|&c| c != 'A').unwrap_or(s.len());
    i += s[i..]
        .iter()
        .position(|&c| c != 'B')
        .unwrap_or(s[i..].len());
    i += s[i..]
        .iter()
        .position(|&c| c != 'C')
        .unwrap_or(s[i..].len());
    let yes = i == s.len();
    println!("{}", if yes { "Yes" } else { "No" });
}
