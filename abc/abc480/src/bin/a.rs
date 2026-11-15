use proconio::{input, marker::Chars};

fn main() {
    input! {
        s1: Chars,
        s2: Chars,
    }
    let yes = !((s1[0] == 'A' && s2[1] == 'D')
        || (s1[0] == 'D' && s2[1] == 'A')
        || (s1[1] == 'A' && s2[0] == 'D')
        || (s1[1] == 'D' && s2[0] == 'A'));
    println!("{}", if yes { "Yes" } else { "No" });
}
