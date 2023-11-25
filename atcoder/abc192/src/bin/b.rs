use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let yes = s.iter().enumerate().all(|(i, &c)| {
        ((i % 2 == 0) && c.is_ascii_lowercase()) || ((i % 2 == 1) && c.is_ascii_uppercase())
    });
    println!("{}", if yes { "Yes" } else { "No" });
}
