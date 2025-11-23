use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let yes = (0..n).all(|i| (i % 2 == 0 && s[i] == 'M') || (i % 2 == 1 && s[i] == 'F'))
        || (0..n).all(|i| (i % 2 == 0 && s[i] == 'F') || (i % 2 == 1 && s[i] == 'M'));
    println!("{}", if yes { "Yes" } else { "No" });
}
