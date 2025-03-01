use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut v = vec![];
    for &c in &s {
        if let Some(&c0) = v.last() {
            if (c0 == '(' && c == ')') || (c0 == '[' && c == ']') || (c0 == '<' && c == '>') {
                v.pop();
                continue;
            }
        }
        v.push(c);
    }
    let yes = v.len() == 0;
    println!("{}", if yes { "Yes" } else { "No" });
}
