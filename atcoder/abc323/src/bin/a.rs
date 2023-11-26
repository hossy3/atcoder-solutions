use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let yes = s.iter().enumerate().all(|(i, &c)| i % 2 == 0 || c == '0');
    println!("{}", if yes { "Yes" } else { "No" });
}
