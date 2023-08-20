use proconio::{input, marker::Chars};

fn f(a: char, b: char) -> bool {
    a == b || a == '?' || b == '?'
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut v0 = vec![false; t.len() + 1]; // from left
    let mut v1 = vec![false; t.len() + 1]; // from right

    v0[0] = true;
    for i in 0..t.len() {
        if !f(s[i], t[i]) {
            break;
        }
        v0[i + 1] = true;
    }

    v1[0] = true;
    for i in 0..t.len() {
        if !f(s[s.len() - i - 1], t[t.len() - i - 1]) {
            break;
        }
        v1[i + 1] = true;
    }

    for i in 0..=t.len() {
        let yes = v0[i] && v1[t.len() - i];
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
