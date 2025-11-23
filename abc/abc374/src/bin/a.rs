use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let yes = s.ends_with(&['s', 'a', 'n']);
    println!("{}", if yes { "Yes" } else { "No" });
}
