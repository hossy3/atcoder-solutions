use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let yes = s.iter().any(|&c| c == 'o') && s.iter().all(|&c| c != 'x');
    println!("{}", if yes { "Yes" } else { "No" });
}
