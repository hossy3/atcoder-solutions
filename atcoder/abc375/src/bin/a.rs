use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let result = (0..n.saturating_sub(2)).filter(|&i| s[i] == '#' && s[i + 1] == '.' && s[i + 2] == '#').count();
    println!("{result}");
}
