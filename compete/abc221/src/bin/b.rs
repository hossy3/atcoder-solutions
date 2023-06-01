use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut v = vec![];
    for i in 0..s.len() {
        if s[i] != t[i] {
            v.push(i);
        }
    }
    let yes = v.len() == 0
        || (v.len() == 2 && v[0] + 1 == v[1] && s[v[0]] == t[v[1]] && s[v[1]] == t[v[0]]);
    println!("{}", if yes { "Yes" } else { "No" });
}
