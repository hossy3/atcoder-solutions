use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let i = s.iter().position(|&c| c == '1').unwrap();
    let yes = i % 2 == 0;
    println!("{}", if yes { "Takahashi" } else { "Aoki" });
}
