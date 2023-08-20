use proconio::{input, marker::Chars};

fn f(s: &[char]) -> bool {
    if s.len() != 8 {
        return false;
    }
    if s[0] < 'A' || s[0] > 'Z' {
        return false;
    }
    if s[1] < '1' || s[1] > '9' {
        return false;
    }
    for i in 2..=6 {
        if s[i] < '0' || s[i] > '9' {
            return false;
        }
    }
    if s[7] < 'A' || s[7] > 'Z' {
        return false;
    }
    true
}

fn main() {
    input! {
        s: Chars,
    }
    let yes = f(&s);
    println!("{}", if yes { "Yes" } else { "No" });
}
