use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        x: char,
        s: [Chars; n],
    }

    let i = x as usize - 'A' as usize;
    let yes = s.iter().any(|s| s[i] == 'o');
    println!("{}", if yes { "Yes" } else { "No" });
}
