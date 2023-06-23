use proconio::{input, marker::Chars};

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
    }
    let yes = !((s1[0] == '.' && s2[1] == '.') || (s1[1] == '.' && s2[0] == '.'));
    println!("{}", if yes { "Yes" } else { "No" });
}
