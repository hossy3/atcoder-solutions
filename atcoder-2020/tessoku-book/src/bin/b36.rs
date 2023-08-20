use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        k: usize,
        s: Chars,
    }
    let n = s.iter().filter(|&&c| c == '1').count();
    let yes = (n % 2) == (k % 2);
    println!("{}", if yes { "Yes" } else { "No" });
}
