use proconio::{input, marker::Chars};

fn is_lower(c: char) -> bool {
    'a' <= c && c <= 'z'
}

fn main() {
    input! {
        s: Chars,
    }
    let yes = !is_lower(s[0]) && s[1..].iter().all(|&c| is_lower(c));
    println!("{}", if yes { "Yes" } else { "No" });
}
