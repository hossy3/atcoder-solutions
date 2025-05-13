use proconio::{input, marker::Chars};

fn f(t: &[char], u: &[char]) -> bool {
    if t.len() < u.len() {
        return false;
    }
    for i in 0..(u.len()) {
        if t[i] != '?' && t[i] != u[i] {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        t: Chars,
        u: Chars,
    }
    let yes = (0..=(t.len())).any(|i| f(&t[i..], &u));
    println!("{}", if yes { "Yes" } else { "No" });
}
